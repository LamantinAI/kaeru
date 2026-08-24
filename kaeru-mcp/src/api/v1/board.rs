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

use crate::api::ApiState;
use crate::api::egress::{redact_excerpt, redact_name};
use crate::api::principal::Principal;

#[derive(Debug, Deserialize)]
pub struct BoardQuery {
    /// Which initiative's board. Required — a board is always one initiative's.
    initiative: String,
    /// Unix seconds to rewind to. Omit for the board as it stands now.
    when: Option<f64>,
    /// Answer with the registry alone. A client that only needs to know what
    /// the columns *are* — to render an empty board, or to merge several
    /// initiatives' vocabularies — should not be sent every card to get there.
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
    fn from_view(view: BoardView, columns_only: bool) -> Self {
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
            let mut out = BoardOut::from_view(view, q.columns.unwrap_or(false));
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
