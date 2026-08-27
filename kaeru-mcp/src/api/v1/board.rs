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
use kaeru_core::{
    BoardColumn, BoardView, DEFAULT_STATUSES, board_node_id, board_view_at, read_node_full,
};
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
    /// `as-configured` when these are the initiative's own columns,
    /// `built-in` when its registry was withheld and the default vocabulary
    /// stood in. Said out loud rather than left to be inferred: a client
    /// drawing "Open / In Progress / Done" should be able to tell whether that
    /// is the initiative's choice or a substitution.
    registry: &'static str,
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

/// Re-buckets a board against the built-in vocabulary, for when the
/// initiative's authored registry may not leave.
///
/// The cards are unaffected — they pass their own visibility check further
/// down — but they have to land somewhere, so they are bucketed by the same
/// rule the core view uses: a status matching no column falls into the first
/// one rather than disappearing. This is the board a client would have seen
/// before the initiative ever customised it.
fn to_default_vocabulary(view: BoardView) -> BoardView {
    let mut columns: Vec<BoardColumn> = DEFAULT_STATUSES
        .iter()
        .map(|(k, l)| BoardColumn {
            key: (*k).to_string(),
            label: (*l).to_string(),
            tasks: Vec::new(),
        })
        .collect();
    for task in view.columns.into_iter().flat_map(|c| c.tasks) {
        let at = columns
            .iter()
            .position(|c| c.key == task.status)
            .unwrap_or(0);
        columns[at].tasks.push(task);
    }
    BoardView {
        initiative: view.initiative,
        columns,
    }
}

impl BoardOut {
    fn from_view(view: BoardView, columns_only: bool, cfg: &ApiConfig) -> Self {
        BoardOut {
            initiative: view.initiative,
            when: None,
            registry: "as-configured",
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
    let cfg = st.cfg.clone();
    let result = tokio::task::spawn_blocking(move || {
        let view = board_view_at(&store, &initiative, when)?;
        // The registry is content, not structure. Its labels live in the
        // `Board` node's `properties` — the one field the pre-share guard
        // never scans — and a status vocabulary describes a workflow, which is
        // sometimes the confidential part ("blocked on legal", a partner's
        // name as a column). So an authored registry leaves only if its node
        // may leave.
        let authored_may_leave = match board_node_id(&store, &initiative, when)? {
            // No board node: the columns are the built-in defaults, which
            // nobody wrote and which therefore say nothing.
            None => true,
            Some(id) => read_node_full(&store, &id)?
                .map(|n| cfg.may_show(&n.visibility))
                .unwrap_or(false),
        };
        Ok::<_, kaeru_core::Error>((view, authored_may_leave))
    })
    .await;

    let mut resp = match result {
        Ok(Ok((view, authored_may_leave))) => {
            let view = if authored_may_leave {
                view
            } else {
                to_default_vocabulary(view)
            };
            let mut out = BoardOut::from_view(view, q.columns.unwrap_or(false), &st.cfg);
            out.when = q.when;
            out.registry = if authored_may_leave {
                "as-configured"
            } else {
                "built-in"
            };
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

#[cfg(test)]
mod registry_visibility_tests {
    use kaeru_core::Store;

    use crate::api::ApiConfig;

    fn cfg(include_local: bool) -> ApiConfig {
        ApiConfig {
            allow: vec!["t".into()],
            include_local,
            ..ApiConfig::default()
        }
    }

    /// A customised registry is content: its labels live in the `Board` node's
    /// `properties`, the one field the pre-share guard never scans, and a
    /// status vocabulary describes a workflow — sometimes the confidential
    /// part. So it leaves only if its node may leave.
    #[tokio::test]
    async fn a_withheld_registry_falls_back_to_the_built_in_vocabulary() {
        let store = Store::open_in_memory().expect("open");
        store.use_initiative("t");
        kaeru_core::ensure_board(&store, "t").expect("board");
        kaeru_core::add_status(&store, "t", "blocked-on-legal", "Blocked on legal").expect("add");

        let board_id = kaeru_core::board_node_id(&store, "t", None)
            .expect("read")
            .expect("exists");
        let visibility = kaeru_core::read_node_full(&store, &board_id)
            .expect("read")
            .expect("exists")
            .visibility;
        assert_eq!(
            visibility, "local",
            "a board node is local like anything else"
        );

        // The gate the handler applies, exercised directly.
        assert!(
            !cfg(false).may_show(&visibility),
            "an authored registry does not leave a shared-only surface"
        );
        assert!(
            cfg(true).may_show(&visibility),
            "and does when the operator opted local content in"
        );

        // And the fallback keeps every card, bucketing an unknown status into
        // the first column rather than dropping it.
        let view = kaeru_core::board_view_at(&store, "t", None).expect("view");
        assert!(
            view.columns.iter().any(|c| c.key == "blocked-on-legal"),
            "the authored column exists locally"
        );
        let fallback = super::to_default_vocabulary(view);
        assert_eq!(fallback.columns.len(), 3, "built-in vocabulary");
        assert!(
            !fallback.columns.iter().any(|c| c.label.contains("legal")),
            "and the authored label is gone: {:?}",
            fallback
                .columns
                .iter()
                .map(|c| &c.label)
                .collect::<Vec<_>>()
        );
    }

    /// Cards survive the substitution — they pass their own visibility check
    /// further down, and a card whose status matches no built-in column falls
    /// into the first rather than disappearing.
    #[tokio::test]
    async fn the_fallback_keeps_every_card() {
        let store = Store::open_in_memory().expect("open");
        store.use_initiative("t");
        kaeru_core::ensure_board(&store, "t").expect("board");
        kaeru_core::add_status(&store, "t", "blocked-on-legal", "Blocked on legal").expect("add");
        let id = kaeru_core::write_task(&store, "the awkward one", None).expect("task");
        kaeru_core::set_status(&store, "t", &id, "blocked-on-legal").expect("status");

        let view = kaeru_core::board_view_at(&store, "t", None).expect("view");
        let before: usize = view.columns.iter().map(|c| c.tasks.len()).sum();
        let after = super::to_default_vocabulary(view);
        let kept: usize = after.columns.iter().map(|c| c.tasks.len()).sum();
        assert_eq!(before, kept, "no card is lost in the substitution");
        assert_eq!(
            after.columns[0].tasks.len(),
            1,
            "unknown status → first column"
        );
    }
}
