//! `GET /v1/board` — the task board for one initiative.
//!
//! This is the `board` verb over plain HTTP, and it exists because the
//! visualizer could not ask the question. The whole-graph export carries a
//! task's `status:` tag but not the `Board` node's `properties`, so the
//! browser had no way to learn an initiative's column registry and drew the
//! built-in vocabulary instead — a board that showed *open / in progress /
//! done* whatever the initiative had actually configured.
//!
//! Columns come back in registry order, empty ones included. That is the
//! board's contract, not a rendering detail: a column with nothing in it is a
//! statement about the work, and dropping it would quietly rewrite the
//! vocabulary the initiative chose.
//!
//! `when` rewinds columns *and* cards together, because a board read at a past
//! moment where today's columns held yesterday's tasks would describe a board
//! that never existed.

use axum::extract::{Query, State};
use axum::http::{StatusCode, header};
use axum::response::{IntoResponse, Response};
use kaeru_core::{BoardView, board_view_at};
use serde::{Deserialize, Serialize};

use crate::api::egress::{redact_excerpt, redact_name};
use crate::api::principal::Principal;
use crate::api::{ApiConfig, ApiState};

#[derive(Debug, Deserialize)]
pub struct BoardQuery {
    /// Which initiative's board. Required — a board is always one initiative's.
    initiative: String,
    /// Unix seconds to rewind to. Omit for the board as it stands now.
    when: Option<f64>,
    /// Answer with the registry alone. A client that only needs to know what
    /// the columns *are* — to render an empty board, or to merge several
    /// initiatives' vocabularies — should not be sent every card to get there.
    ///
    /// This is about the wire, not the query: the cards are still read and
    /// built before being dropped. Worth knowing before anyone reads the
    /// saving as a claim about what the server does.
    columns: Option<bool>,
}

/// The wire shape.
///
/// Deliberately not `BoardView` with a `Serialize` derive bolted on: a rename
/// inside `kaeru-core` would then silently become a breaking change for every
/// client. This is the contract, and it is written down here.
#[derive(Debug, Serialize)]
pub struct BoardOut {
    initiative: String,
    /// Echoed back so a client can tell a rewound board from a live one
    /// without keeping track of what it asked for.
    when: Option<f64>,
    columns: Vec<ColumnOut>,
}

#[derive(Debug, Serialize)]
struct ColumnOut {
    key: String,
    label: String,
    tasks: Vec<TaskOut>,
}

#[derive(Debug, Serialize)]
struct TaskOut {
    id: String,
    name: String,
    excerpt: Option<String>,
    /// `due:YYYY-MM-DD`, already parsed off the tag.
    due: Option<String>,
    /// Assertion time, for the time-lapse scrubber.
    ts: Option<f64>,
    /// True when the guard replaced this card's text. The card still appears —
    /// see `egress::redact_name`.
    redacted: bool,
}

impl BoardOut {
    fn from_view(view: BoardView, columns_only: bool, cfg: &ApiConfig) -> Self {
        BoardOut {
            initiative: view.initiative,
            when: None,
            columns: view
                .columns
                .into_iter()
                .map(|c| ColumnOut {
                    key: c.key,
                    label: c.label,
                    tasks: if columns_only { Vec::new() } else { c.tasks }
                        .into_iter()
                        // A card the operator has not shared does not appear.
                        // Dropping rather than redacting matches what the
                        // ceiling does one level up, and matches what `at` and
                        // `export` answer for the very same node — before this
                        // check the board handed over in full what those two
                        // refused outright, on one daemon under one config.
                        .filter(|t| cfg.may_show(&t.visibility))
                        .map(|t| {
                            let (name, name_hit) = redact_name(&t.name, "task");
                            let (excerpt, body_hit) = redact_excerpt(t.body_excerpt.as_deref());
                            TaskOut {
                                id: t.id.to_string(),
                                name,
                                excerpt,
                                due: t.due,
                                ts: t.ts,
                                redacted: name_hit || body_hit,
                            }
                        })
                        .collect(),
                })
                .collect(),
        }
    }
}

pub async fn board(
    _who: Principal,
    State(st): State<ApiState>,
    Query(q): Query<BoardQuery>,
) -> Response {
    // Outside the operator's ceiling the answer is "no such board", not "you
    // may not see that board" — see `ApiConfig::reaches`.
    if !st.cfg.reaches(&q.initiative) {
        let mut resp = (StatusCode::NOT_FOUND, "no such board").into_response();
        st.cfg.finish(&mut resp);
        return resp;
    }

    let store = st.store.clone();
    let initiative = q.initiative.clone();
    let when = q.when;
    let result =
        tokio::task::spawn_blocking(move || board_view_at(&store, &initiative, when)).await;

    let mut resp = match result {
        Ok(Ok(view)) => {
            let mut out = BoardOut::from_view(view, q.columns.unwrap_or(false), &st.cfg);
            out.when = q.when;
            match serde_json::to_string(&out) {
                Ok(json) => (
                    StatusCode::OK,
                    [(header::CONTENT_TYPE, "application/json")],
                    json,
                )
                    .into_response(),
                Err(e) => internal(format!("serialize board: {e}")),
            }
        }
        Ok(Err(e)) => internal(format!("read board: {e}")),
        Err(e) => internal(format!("board task: {e}")),
    };
    st.cfg.finish(&mut resp);
    resp
}

fn internal(msg: String) -> Response {
    tracing::warn!(error = %msg, "board read failed");
    (StatusCode::INTERNAL_SERVER_ERROR, msg).into_response()
}

#[cfg(test)]
mod tests {
    use kaeru_core::{Store, Visibility, set_visibility, write_task};

    use super::BoardOut;
    use crate::api::ApiConfig;
    use kaeru_core::board_view;

    fn open_ceiling() -> ApiConfig {
        ApiConfig {
            allow: vec!["t".into()],
            ..ApiConfig::default()
        }
    }

    /// The defect this test exists for: the board consulted the operator's
    /// initiative ceiling and nothing else, so it served `local` cards — the
    /// default visibility — that `at` and `export` refused on the same daemon
    /// under the same config.
    #[test]
    fn a_local_card_never_reaches_the_wire() {
        let store = Store::open_in_memory().expect("open");
        store.use_initiative("t");
        write_task(&store, "a private chore", None).expect("task");

        let view = board_view(&store, "t").expect("board");
        assert_eq!(
            view.columns.iter().map(|c| c.tasks.len()).sum::<usize>(),
            1,
            "the vault has the card"
        );

        let out = BoardOut::from_view(view, false, &open_ceiling());
        let shown: usize = out.columns.iter().map(|c| c.tasks.len()).sum();
        assert_eq!(shown, 0, "and the board does not hand it over");
    }

    #[test]
    fn a_shared_card_does() {
        let store = Store::open_in_memory().expect("open");
        store.use_initiative("t");
        let id = write_task(&store, "a chore worth sharing", None).expect("task");
        set_visibility(&store, &id, Visibility::Shared).expect("share");

        let out = BoardOut::from_view(
            board_view(&store, "t").expect("board"),
            false,
            &open_ceiling(),
        );
        let names: Vec<&str> = out
            .columns
            .iter()
            .flat_map(|c| c.tasks.iter().map(|t| t.name.as_str()))
            .collect();
        // `write_task` slugs the name it was given
        assert_eq!(names.len(), 1);
        assert!(
            names[0].starts_with("a-chore-worth-sharing"),
            "got {names:?}"
        );
    }

    /// `include_local` is the operator saying "this daemon may show my own
    /// notes" — the same card, the same config but for that one flag.
    #[test]
    fn include_local_is_what_lets_it_through() {
        let store = Store::open_in_memory().expect("open");
        store.use_initiative("t");
        write_task(&store, "a private chore", None).expect("task");

        let cfg = ApiConfig {
            include_local: true,
            ..open_ceiling()
        };
        let out = BoardOut::from_view(board_view(&store, "t").expect("board"), false, &cfg);
        assert_eq!(out.columns.iter().map(|c| c.tasks.len()).sum::<usize>(), 1);
    }
}
