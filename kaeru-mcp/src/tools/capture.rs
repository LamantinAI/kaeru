//! Write-side tools: `episode`, `jot`, `link`, `unlink`, `cite`.
//!
//! The capture verbs (`episode` / `jot` / `cite`) take an optional
//! `visibility`. With `visibility: shared` the freshly-created node is
//! pushed to the team cloud in the **same call** — gated exactly like
//! `share` (initiative policy + secret guard). The local `shared` flag is
//! set only after the cloud accepts it, so it never marks a node shared
//! that isn't actually in the cloud. `link` / `unlink` stay purely local.

use kaeru_core::{EdgeType, EpisodeKind, Significance, Store};
use rmcp::ErrorData as McpError;
use rmcp::model::CallToolResult;

use crate::cloud_client::{CloudClient, CloudRegistry};
use crate::tools::cloud::{EdgeChange, propagate_edge, push_to_cloud};
use crate::utils::{
    CaptureLink, apply_reminder, arrival_note, capture_result, link_at_capture, markup_strip_note,
    parse_layer, parse_wants_shared, resolve_link_endpoint, text, to_mcp, with_initiative,
};

/// When `want_share`, attempts to push the just-created node `id` to the
/// cloud and appends the outcome to `msg`. Needs both a configured cloud
/// and an initiative (sharing policy is per-initiative); absent either, it
/// notes that the node stayed local.
async fn maybe_share(
    store: &Store,
    cloud: Option<&CloudClient>,
    id: &str,
    initiative: Option<&str>,
    want_share: bool,
    msg: &mut String,
) -> Result<(), McpError> {
    if !want_share {
        return Ok(());
    }
    match (cloud, initiative) {
        (Some(c), Some(init)) => {
            let outcome = push_to_cloud(store, c, id, init, false).await?;
            msg.push('\n');
            msg.push_str(&outcome);
        }
        (None, _) => {
            msg.push_str("\n(shared requested, but cloud not configured — saved local)");
        }
        (_, None) => {
            msg.push_str(
                "\n(shared requested, but no initiative — saved local; pass initiative to share)",
            );
        }
    }
    Ok(())
}

pub async fn episode(
    store: &Store,
    cloud: Option<&CloudClient>,
    name: &str,
    body: &str,
    layer: Option<&str>,
    visibility: Option<&str>,
    after: Option<&str>,
    for_days: Option<i64>,
    initiative: Option<&str>,
    link: CaptureLink<'_>,
) -> Result<CallToolResult, McpError> {
    let want_share = parse_wants_shared(visibility)?;
    // Before the write: afterwards the initiative always has a node (#86).
    let arrival = arrival_note(store, initiative);
    let id = with_initiative(store, initiative, || {
        let layer = parse_layer(layer)?;
        kaeru_core::write_episode_with_layer(
            store,
            EpisodeKind::Observation,
            Significance::Medium,
            name,
            body,
            layer,
        )
        .map_err(to_mcp)
    })?;
    let linked = with_initiative(store, initiative, || Ok(link_at_capture(store, &id, link)))?;
    let reminder = apply_reminder(store, &id, after, for_days)?;
    let mut msg = format!("wrote episode: {name} — {id}");
    if let Some(note) = markup_strip_note(&[("name", name), ("body", body)]) {
        msg.push_str(&note);
    }
    msg.push_str(&linked);
    maybe_share(store, cloud, &id, initiative, want_share, &mut msg).await?;
    if let Some(note) = &reminder {
        msg.push_str(note);
    }
    if let Some(note) = arrival {
        msg.push_str(&note);
    }
    Ok(capture_result(store, &id, initiative, &msg))
}

pub async fn jot(
    store: &Store,
    cloud: Option<&CloudClient>,
    body: &str,
    layer: Option<&str>,
    visibility: Option<&str>,
    after: Option<&str>,
    for_days: Option<i64>,
    initiative: Option<&str>,
    link: CaptureLink<'_>,
) -> Result<CallToolResult, McpError> {
    let want_share = parse_wants_shared(visibility)?;
    let arrival = arrival_note(store, initiative);
    let id = with_initiative(store, initiative, || {
        let layer = parse_layer(layer)?;
        kaeru_core::jot_with_layer(store, body, layer).map_err(to_mcp)
    })?;
    let name = kaeru_core::node_brief_by_id(store, &id)
        .ok()
        .flatten()
        .map(|b| b.name)
        .unwrap_or_default();
    let linked = with_initiative(store, initiative, || Ok(link_at_capture(store, &id, link)))?;
    let reminder = apply_reminder(store, &id, after, for_days)?;
    let mut msg = format!("jotted: {name} — {id}");
    if let Some(note) = markup_strip_note(&[("body", body)]) {
        msg.push_str(&note);
    }
    msg.push_str(&linked);
    maybe_share(store, cloud, &id, initiative, want_share, &mut msg).await?;
    if let Some(note) = &reminder {
        msg.push_str(note);
    }
    if let Some(note) = arrival {
        msg.push_str(&note);
    }
    Ok(text(&msg))
}

/// The three graph verbs are `async` and cloud-aware for one reason: an edge
/// between two shared nodes belongs to the cloud as much as the nodes do, and
/// none of them had any path to HTTP — so the cloud's copy of the graph was
/// frozen at the moment each node was shared (#85). The local write always
/// happens first and is never conditional on the network.
#[allow(clippy::too_many_arguments)]
pub async fn link(
    store: &Store,
    clouds: &CloudRegistry,
    cloud_name: Option<&str>,
    from: &str,
    to: &str,
    edge_type_str: &str,
    weight: f64,
    initiative: Option<&str>,
) -> Result<CallToolResult, McpError> {
    let (edge, from_id, to_id) = with_initiative(store, initiative, || {
        let edge: EdgeType = edge_type_str.parse().map_err(to_mcp)?;
        let from_id = resolve_link_endpoint(store, from)?;
        let to_id = resolve_link_endpoint(store, to)?;
        // Weight is required and stated by the caller — it is the signal chain
        // shortest-paths route on, so there is no neutral default to fall back
        // to. `link_with_weight` clamps to 0..1.
        kaeru_core::link_with_weight(store, &from_id, &to_id, edge, weight).map_err(to_mcp)?;
        Ok((edge, from_id, to_id))
    })?;
    let mut msg = format!(
        "linked: {from} -[{}]-> {to} (weight {weight:.2})",
        edge.as_str()
    );
    propagate_edge(
        store,
        clouds,
        cloud_name,
        &from_id,
        &to_id,
        edge,
        EdgeChange::Upsert(weight.clamp(0.0, 1.0)),
        initiative,
        &mut msg,
    )
    .await?;
    Ok(text(&msg))
}

#[allow(clippy::too_many_arguments)]
pub async fn unlink(
    store: &Store,
    clouds: &CloudRegistry,
    cloud_name: Option<&str>,
    from: &str,
    to: &str,
    edge_type_str: &str,
    initiative: Option<&str>,
) -> Result<CallToolResult, McpError> {
    let (edge, from_id, to_id) = with_initiative(store, initiative, || {
        let edge: EdgeType = edge_type_str.parse().map_err(to_mcp)?;
        let from_id = resolve_link_endpoint(store, from)?;
        let to_id = resolve_link_endpoint(store, to)?;
        kaeru_core::unlink(store, &from_id, &to_id, edge).map_err(to_mcp)?;
        Ok((edge, from_id, to_id))
    })?;
    let mut msg = format!("unlinked: {from} -[{}]-> {to}", edge.as_str());
    propagate_edge(
        store,
        clouds,
        cloud_name,
        &from_id,
        &to_id,
        edge,
        EdgeChange::Retract,
        initiative,
        &mut msg,
    )
    .await?;
    Ok(text(&msg))
}

/// Sets the connection strength (`weight`, 0..1) of an existing edge —
/// in-place, no new version. Stronger edges make shorter knowledge-chain
/// paths. Use to tune which links matter after the fact.
#[allow(clippy::too_many_arguments)]
pub async fn reweight(
    store: &Store,
    clouds: &CloudRegistry,
    cloud_name: Option<&str>,
    from: &str,
    to: &str,
    edge_type_str: &str,
    weight: f64,
    initiative: Option<&str>,
) -> Result<CallToolResult, McpError> {
    let (edge, from_id, to_id) = with_initiative(store, initiative, || {
        let edge: EdgeType = edge_type_str.parse().map_err(to_mcp)?;
        let from_id = resolve_link_endpoint(store, from)?;
        let to_id = resolve_link_endpoint(store, to)?;
        kaeru_core::set_edge_weight(store, &from_id, &to_id, edge, weight).map_err(to_mcp)?;
        Ok((edge, from_id, to_id))
    })?;
    let mut msg = format!(
        "reweighted: {from} -[{}]-> {to} = {:.2}",
        edge.as_str(),
        weight.clamp(0.0, 1.0)
    );
    propagate_edge(
        store,
        clouds,
        cloud_name,
        &from_id,
        &to_id,
        edge,
        EdgeChange::Upsert(weight.clamp(0.0, 1.0)),
        initiative,
        &mut msg,
    )
    .await?;
    Ok(text(&msg))
}

pub async fn cite(
    store: &Store,
    cloud: Option<&CloudClient>,
    name: &str,
    url: Option<&str>,
    body: &str,
    layer: Option<&str>,
    visibility: Option<&str>,
    after: Option<&str>,
    for_days: Option<i64>,
    initiative: Option<&str>,
    link: CaptureLink<'_>,
) -> Result<CallToolResult, McpError> {
    let want_share = parse_wants_shared(visibility)?;
    let arrival = arrival_note(store, initiative);
    let id = with_initiative(store, initiative, || {
        let layer = parse_layer(layer)?;
        kaeru_core::cite_with_layer(store, name, url, body, layer).map_err(to_mcp)
    })?;
    let linked = with_initiative(store, initiative, || Ok(link_at_capture(store, &id, link)))?;
    let reminder = apply_reminder(store, &id, after, for_days)?;
    let mut msg = match url {
        Some(u) => format!("cited: {name} ({u}) — {id}"),
        None => format!("cited: {name} — {id}"),
    };
    if let Some(note) = markup_strip_note(&[("name", name), ("body", body)]) {
        msg.push_str(&note);
    }
    msg.push_str(&linked);
    maybe_share(store, cloud, &id, initiative, want_share, &mut msg).await?;
    if let Some(note) = &reminder {
        msg.push_str(note);
    }
    if let Some(note) = arrival {
        msg.push_str(&note);
    }
    Ok(capture_result(store, &id, initiative, &msg))
}

#[cfg(test)]
mod tests {
    use kaeru_core::{EpisodeKind, Significance, Store};

    use rmcp::model::CallToolResult;

    use super::{CaptureLink, CloudRegistry, episode, jot, link};

    fn text_of(r: CallToolResult) -> String {
        r.content
            .iter()
            .filter_map(|c| c.as_text().map(|t| t.text.clone()))
            .collect::<Vec<_>>()
            .join("")
    }

    fn store_t() -> Store {
        let store = Store::open_in_memory().expect("open");
        store.use_initiative("t");
        store
    }

    /// Seeds a node under `initiative` and returns its id.
    fn seed(store: &Store, initiative: &str, name: &str) -> String {
        store.use_initiative(initiative);
        kaeru_core::write_episode(
            store,
            EpisodeKind::Observation,
            Significance::Low,
            name,
            "body",
        )
        .expect("write")
    }

    /// #81: a `core` node with no initiative is refused at the source — it could
    /// never keep the "loads every session" promise, so it must not be written.
    /// Cross-project reach is `attach`, not the absence of an initiative.
    #[tokio::test]
    async fn core_without_initiative_is_refused() {
        let store = Store::open_in_memory().expect("open");
        let res = episode(
            &store,
            None,
            "screenshot-rule",
            "use the other window",
            Some("core"),
            None,
            None,
            None,
            None,
            CaptureLink::default(),
        )
        .await;
        assert!(res.is_err(), "a core node with no initiative is refused");
    }

    /// A `core` node WITH an initiative is exactly right.
    #[tokio::test]
    async fn core_with_an_initiative_is_accepted() {
        let store = Store::open_in_memory().expect("open");
        let res = episode(
            &store,
            None,
            "real-rule",
            "body",
            Some("core"),
            None,
            None,
            None,
            Some("proj"),
            CaptureLink::default(),
        )
        .await;
        assert!(res.is_ok(), "core is fine once it has a home");
    }

    /// The refusal is specific to `core`; an ordinary un-tagged note is fine.
    #[tokio::test]
    async fn a_warm_note_without_initiative_is_accepted() {
        let store = Store::open_in_memory().expect("open");
        let res = episode(
            &store,
            None,
            "note",
            "body",
            None,
            None,
            None,
            None,
            None,
            CaptureLink::default(),
        )
        .await;
        assert!(res.is_ok(), "warm/hot untagged capture stays allowed");
    }

    /// Counts edges between two ids cross-initiative — a cross-initiative
    /// edge is invisible to a scoped `between` (which requires both endpoints
    /// in the active initiative), so read it with the scope cleared.
    fn edge_count(store: &Store, a: &str, b: &str) -> usize {
        store
            .scoped(None, |s| {
                kaeru_core::between(s, &a.to_string(), &b.to_string())
            })
            .expect("between")
            .len()
    }

    /// A registry with nothing in it — the local-only daemon, and what every
    /// test here runs against: no cloud means `propagate_edge` returns before
    /// it touches the network.
    fn no_clouds() -> CloudRegistry {
        CloudRegistry::new(std::collections::HashMap::new(), None)
    }

    /// An edge can join nodes living under different initiatives: scoped to
    /// `a`, the source resolves in-scope while the destination (only in `b`)
    /// resolves through the cross-initiative fallback. This is the friction
    /// that used to force dropping the initiative scope to link at all.
    #[tokio::test]
    async fn link_joins_nodes_across_initiatives() {
        let store = Store::open_in_memory().expect("open");
        let a = seed(&store, "a", "node-a");
        let b = seed(&store, "b", "node-b");

        link(
            &store,
            &no_clouds(),
            None,
            "node-a",
            "node-b",
            "refers_to",
            0.5,
            Some("a"),
        )
        .await
        .expect("cross-initiative link resolves");

        assert_eq!(edge_count(&store, &a, &b), 1, "edge was created");
    }

    /// Endpoints may be raw UUIDv7 ids, not just names.
    #[tokio::test]
    async fn link_accepts_ids() {
        let store = Store::open_in_memory().expect("open");
        let a = seed(&store, "x", "src");
        let b = seed(&store, "x", "dst");

        link(
            &store,
            &no_clouds(),
            None,
            &a,
            &b,
            "refers_to",
            0.5,
            Some("x"),
        )
        .await
        .expect("link by id resolves");

        assert_eq!(edge_count(&store, &a, &b), 1, "edge was created from ids");
    }

    /// The change the report turns on: an initiative is the one vocabulary any
    /// string can join silently, and writing under a name that does not exist
    /// created it with no confirmation and no comparison against what was
    /// already there (#86). It says so now, and prints the list — which is the
    /// only thing that reaches the case no string metric can, an alias in
    /// another script.
    #[tokio::test]
    async fn a_write_that_creates_an_initiative_says_so_and_lists_the_others() {
        let store = Store::open_in_memory().expect("open");
        seed(&store, "n8n-agents", "an-existing-note");

        let out = jot(
            &store,
            None,
            "a lesson note",
            None,
            None,
            None,
            None,
            Some("kurs-agentov"),
            CaptureLink::default(),
        )
        .await
        .expect("jot");
        let rendered = format!("{:?}", out.content);

        assert!(
            rendered.contains("`kurs-agentov` is new"),
            "the arrival is named: {rendered}"
        );
        assert!(
            rendered.contains("n8n-agents"),
            "and the established name is on screen to be recognised: {rendered}"
        );
    }

    /// A near miss gets pointed at rather than left to be spotted in a list,
    /// and the message names the verb that undoes it.
    #[tokio::test]
    async fn a_near_miss_is_named_with_the_verb_that_rejoins_it() {
        let store = Store::open_in_memory().expect("open");
        seed(&store, "alpha-beta", "a-note");

        let out = jot(
            &store,
            None,
            "another note",
            None,
            None,
            None,
            None,
            Some("alpha_beta"),
            CaptureLink::default(),
        )
        .await
        .expect("jot");
        let rendered = format!("{:?}", out.content);

        assert!(rendered.contains("Did you mean `alpha-beta`"), "{rendered}");
        assert!(
            rendered.contains("merge_initiative"),
            "and how to fix it: {rendered}"
        );
    }

    /// Writing into an initiative that already exists says nothing — the note
    /// is for arrivals, and a line on every write would be noise.
    #[tokio::test]
    async fn an_ordinary_write_is_not_annotated() {
        let store = Store::open_in_memory().expect("open");
        seed(&store, "alpha", "a-note");

        let out = jot(
            &store,
            None,
            "another note",
            None,
            None,
            None,
            None,
            Some("alpha"),
            CaptureLink::default(),
        )
        .await
        .expect("jot");
        let rendered = format!("{:?}", out.content);

        assert!(!rendered.contains("is new"), "{rendered}");
    }

    /// The measurement behind #102: a vault reached 23 nodes and 0 edges
    /// because linking was a second call and the nudge asked seven times in
    /// a row. The edge is available inside the capture now.
    #[tokio::test]
    async fn a_capture_can_make_its_edge_in_the_same_call() {
        let store = store_t();
        let anchor = kaeru_core::write_episode(
            &store,
            EpisodeKind::Observation,
            Significance::Medium,
            "provider-decision",
            "we chose X",
        )
        .expect("write");
        kaeru_core::attach_node(&store, &anchor, "t").expect("attach");

        let out = text_of(
            episode(
                &store,
                None,
                "provider-switch",
                "and here is why it changed",
                None,
                None,
                None,
                None,
                Some("t"),
                CaptureLink {
                    to: Some("provider-decision"),
                    edge_type: Some("derived_from"),
                    weight: Some(0.9),
                },
            )
            .await
            .expect("episode"),
        );
        assert!(
            out.contains("linked: -[derived_from]-> provider-decision (weight 0.90)"),
            "the edge is reported in the capture's own result:\n{out}"
        );
        // And the nudge that asks for a link is gone, because there is one.
        assert!(
            !out.contains("Don't leave it an island"),
            "no nudge for a node that is not an island:\n{out}"
        );
    }

    /// A mistyped target must not cost the thought. The capture lands, and
    /// the missing edge says so — silence is how a vault goes flat.
    #[tokio::test]
    async fn a_target_that_does_not_resolve_still_keeps_the_capture() {
        let store = store_t();
        let out = text_of(
            episode(
                &store,
                None,
                "a-note",
                "body",
                None,
                None,
                None,
                None,
                Some("t"),
                CaptureLink {
                    to: Some("no-such-node"),
                    edge_type: None,
                    weight: Some(0.5),
                },
            )
            .await
            .expect("episode"),
        );
        assert!(
            out.contains("wrote episode: a-note"),
            "the capture landed:\n{out}"
        );
        assert!(
            out.contains("NOT linked") && out.contains("no-such-node"),
            "and the edge says it was not made:\n{out}"
        );
    }

    /// `weight` has no default here either — the reason it is required on
    /// `link` does not stop applying because the edge is made in a capture.
    #[tokio::test]
    async fn a_link_without_a_weight_is_refused_and_named() {
        let store = store_t();
        let anchor = kaeru_core::write_episode(
            &store,
            EpisodeKind::Observation,
            Significance::Medium,
            "anchor",
            "body",
        )
        .expect("write");
        kaeru_core::attach_node(&store, &anchor, "t").expect("attach");

        let out = text_of(
            jot(
                &store,
                None,
                "a passing thought",
                None,
                None,
                None,
                None,
                Some("t"),
                CaptureLink {
                    to: Some("anchor"),
                    edge_type: None,
                    weight: None,
                },
            )
            .await
            .expect("jot"),
        );
        assert!(
            out.contains("NOT linked") && out.contains("`weight`"),
            "the refusal names what is missing:\n{out}"
        );
    }
}
