// Full live bodies are assembled from the shared read continuation. A baked
// graph is the explicit offline fallback; old daemon endpoints are not probed.
import { memory, snapshotBody } from './api.js'
const cache = new Map()

export async function bodiesFor(ids) {
  for (const id of new Set(ids)) {
    if (!id || cache.has(id)) continue
    try { cache.set(id, await memory.record(id)) }
    catch (_) { const body = snapshotBody(id); if (body != null) cache.set(id, body) }
  }
  return Object.fromEntries(ids.filter((id) => cache.get(id)).map((id) => [id, cache.get(id)]))
}
