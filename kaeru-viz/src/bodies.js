// Node bodies, fetched for the nodes that are about to be shown.
//
// The export truncates every body — it has to, or the whole-graph document
// would be unusable — so a room that wants the real text has to ask for it.
// It used to ask the only way it could: `?bodies=true`, which is every body in
// the vault. On this vault that is 3.7 MB to render three paragraphs.
//
// `/v1/at` is the `at` verb, one node at a time, so a room now pays for what
// it shows. The old fetch is still here as the fallback, because a baked
// snapshot has no daemon behind it and an older daemon has no API.
//
// Caching is per node and negative results count: a node with no body should
// not be asked for twice. Only successful lookups are cached — a transient
// failure has to leave the next call free to retry rather than stranding the
// session on excerpts.

const cache = new Map()        // id → body, or null for "asked, has none"
let bulk = null                // the fallback, fetched at most once
let apiProbe = null

/** Whether the daemon serves `/v1`.
 *
 *  Asked once, and asked in the one way that gives an unambiguous answer:
 *  `/v1/at` without an id is a *bad request* if the route exists and a *not
 *  found* if it does not. Every other probe confuses "no such node" with "no
 *  such endpoint", because both are 404. */
function apiUp() {
  if (!apiProbe) {
    apiProbe = fetch('/v1/at').then((r) => r.status === 400).catch(() => false)
  }
  return apiProbe
}

async function one(id) {
  try {
    const r = await fetch(`/v1/at?id=${encodeURIComponent(id)}`)
    if (!r.ok) return null              // absent, or outside the operator's ceiling
    return (await r.json()).body || null
  } catch (_) { return null }
}

/** Every body in the vault — the old whole-graph fetch, kept for snapshots. */
function bulkBodies() {
  if (bulk) return bulk
  bulk = (async () => {
    for (const url of ['/graph.json?bodies=true', './graph.json?bodies=true']) {
      try {
        const r = await fetch(url)
        if (!r.ok) continue
        const g = await r.json()
        const out = {}
        for (const n of g.nodes) if (n.body) out[n.id] = n.body
        return out
      } catch (_) { /* fall through to the baked snapshot */ }
    }
    bulk = null                          // nothing arrived — let the next call retry
    return {}
  })()
  return bulk
}

/** Resolves to a `{ [nodeId]: body }` map for the ids asked for. Ids with no
 *  body — or no permission to be read — are simply absent from the map. */
export async function bodiesFor(ids) {
  const want = [...new Set(ids)].filter((id) => id && !cache.has(id))
  if (want.length) {
    if (await apiUp()) {
      const got = await Promise.all(want.map(one))
      want.forEach((id, i) => cache.set(id, got[i]))
    } else {
      const all = await bulkBodies()
      for (const id of want) cache.set(id, all[id] || null)
    }
  }
  const out = {}
  for (const id of ids) { const b = cache.get(id); if (b) out[id] = b }
  return out
}
