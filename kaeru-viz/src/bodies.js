// Full node bodies, fetched once and shared by every room that needs them.
//
// The galaxy only ever needs excerpts, so `/graph.json` truncates them. The
// reader and the board both want the whole text, and both want it lazily —
// pay for the heavier payload the first time a room actually opens, not on
// page load.
//
// The cache is only set on success: a transient failure must leave the next
// call free to retry rather than stranding the session on excerpts.

let cached = null
let inFlight = null

/** Resolves to a `{ [nodeId]: body }` map, or `{}` when the daemon is out. */
export function loadFullBodies() {
  if (cached) return Promise.resolve(cached)
  if (inFlight) return inFlight
  inFlight = (async () => {
    for (const url of ['/graph.json?bodies=true', './graph.json?bodies=true']) {
      try {
        const r = await fetch(url)
        if (!r.ok) continue
        const g = await r.json()
        const out = {}
        for (const n of g.nodes) if (n.body) out[n.id] = n.body
        cached = out
        return out
      } catch (_) { /* fall through to the baked snapshot */ }
    }
    inFlight = null          // nothing arrived — let the next open try again
    return {}
  })()
  return inFlight
}
