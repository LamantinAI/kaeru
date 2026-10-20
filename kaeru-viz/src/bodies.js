// Full bodies come only from authorized live read continuations.
import { memory } from './api.js'
const cache = new Map()

export async function bodiesFor(ids) {
  for (const id of new Set(ids)) {
    if (!id || cache.has(id)) continue
    cache.set(id, await memory.record(id))
  }
  return Object.fromEntries(ids.filter((id) => cache.get(id)).map((id) => [id, cache.get(id)]))
}
