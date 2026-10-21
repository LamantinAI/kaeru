// Full bodies come only from authorized live reads in the current graph session.
import { memory, registeredGraph } from './api.js'
const cache = new Map()
let snapshot = null

export async function bodiesFor(ids, options = {}) {
  const graph = registeredGraph()
  const current = graph?.snapshot_id ?? graph
  if (current !== snapshot) { cache.clear(); snapshot = current }
  const result = {}
  for (const id of new Set(ids)) {
    options.signal?.throwIfAborted()
    if (!id) continue
    if (!cache.has(id)) {
      const body = await memory.record(id, options)
      // A new graph/session may have replaced the previous one during the read.
      if (registeredGraph() !== graph) throw new DOMException('Graph changed', 'AbortError')
      cache.set(id, body)
    }
    result[id] = cache.get(id)
  }
  return result
}
