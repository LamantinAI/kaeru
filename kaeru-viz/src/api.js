// The visualizer consumes the same named JSON operations as agents.
import { sessionHeaders } from './session.js'

const scopesByNode = new Map()
let registered = null

export function registerGraph(graph) {
  registered = graph
  scopesByNode.clear()
  for (const node of graph.nodes || []) if (node.initiatives?.length) scopesByNode.set(node.id, node.initiatives[0])
}
export function scopeFor(id) { return scopesByNode.get(id) }
export function registeredGraph() { return registered }

export function createClient({ baseUrl = '', headers = {}, fetcher = fetch, onError } = {}) {
  async function call(operation, arguments_, { signal } = {}) {
    const response = await fetcher(`${baseUrl}/v1/${operation}`, {
      signal, method: 'POST', credentials: 'same-origin', headers: { 'content-type': 'application/json', ...(typeof headers === 'function' ? headers() : headers) },
      body: JSON.stringify(arguments_),
    }).catch((error) => { if (error.name !== 'AbortError') onError?.(error); throw error })
    const value = await response.json().catch((error) => {
      if (error.name === 'AbortError') throw error
      if (response.ok) { onError?.(error); throw error }
      return {}
    })
    if (!response.ok || value.state === 'rejected' || value.state === 'partial') {
      const error = new Error(value.issues?.[0]?.message || `Memory HTTP ${response.status}`)
      error.code = value.issues?.[0]?.code
      error.status = response.status
      if (error.code !== 'CURSOR_STALE') onError?.(error)
      throw error
    }
    return value
  }
  // Gather one coherent read. Restart a stale snapshot a bounded number of
  // times, rather than mixing revisions into a seemingly complete graph.
  async function pages(operation, arguments_, stop = () => false, options = {}) {
    for (let attempt = 0; attempt < 3; attempt++) {
      const out = []
      let next = arguments_
      try {
        for (let page = 0; next && page < 10000; page++) {
          const value = await call(operation, next, options)
          out.push(value)
          if (stop(value)) return out
          next = value.continuation?.next_arguments
        }
        if (next) throw new Error('Memory pagination limit exceeded')
        return out
      } catch (error) {
        if (error.code !== 'CURSOR_STALE') throw error
        if (attempt === 2) { onError?.(error); throw error }
      }
    }
  }
  async function record(id, options = {}) {
    const initiative = scopeFor(id)
    if (!initiative) throw new Error('No visible initiative for this record')
    const responses = await pages('read', { initiative, node: { id }, budget_chars: 64000 }, () => false, options)
    return responses.map((value) => value.data.node.body || '').join('')
  }
  async function graph(selectedScopes, options = {}) {
    const args = { view: 'graph', include: ['created_at'], budget_chars: 64000 }
    if (selectedScopes?.length) args.initiatives = selectedScopes
    const responses = await pages('context', args, () => false, options)
    const snapshot = responses[0].data.graph.snapshot_id
    const nodes = new Map(), edges = new Map(), chains = new Map()
    for (const page of responses) {
      const graph = page.data.graph
      if (graph.snapshot_id !== snapshot) throw new Error('Graph pages belong to different snapshots')
      for (const node of graph.nodes) nodes.set(node.id, { ...node, type: node.node_type,
        created_secs: node.created_at ? Date.parse(node.created_at) / 1000 : null,
        degree: 0, body_loaded: false })
    }
    for (const page of responses) {
      for (const edge of page.data.graph.edges) if (nodes.has(edge.from) && nodes.has(edge.to)) {
        edges.set(`${edge.from}|${edge.to}|${edge.edge_type}`, { src: edge.from, dst: edge.to,
          type: edge.edge_type, weight: edge.weight, dst_store: 'local' })
      }
      for (const member of page.data.graph.members) {
        if (!nodes.has(member.chain_id) || !nodes.has(member.node_id)) throw new Error('Graph chain has an absent endpoint')
        if (!chains.has(member.chain_id)) chains.set(member.chain_id, { id: member.chain_id, name: nodes.get(member.chain_id).name, entries: [] })
        chains.get(member.chain_id).entries.push(member)
      }
    }
    for (const edge of edges.values()) { nodes.get(edge.src).degree++; nodes.get(edge.dst).degree++ }
    const times = [...nodes.values()].map((n) => n.created_secs).filter(Number.isFinite)
    const scopes = responses[0].coverage?.initiatives || [...new Set([...nodes.values()].flatMap((n) => n.initiatives))]
    return { snapshot_id: snapshot, nodes: [...nodes.values()], edges: [...edges.values()],
      chains: [...chains.values()].map(({ entries, ...chain }) => ({ ...chain, members: entries.sort((a, b) => a.position - b.position).map((m) => m.node_id) })), project_links: [],
      initiatives: scopes.map((name) => ({ name, node_count: [...nodes.values()].filter((n) => n.initiatives.includes(name)).length })),
      meta: { node_count: nodes.size, edge_count: edges.size, chain_count: chains.size, initiative_count: scopes.length,
        redacted_count: null, earliest_secs: times.length ? Math.min(...times) : null, latest_secs: times.length ? Math.max(...times) : null } }
  }

  return { call, pages, record, graph }
}
export const memory = createClient({ headers: sessionHeaders, onError(error) {
  if (typeof window !== 'undefined') window.dispatchEvent(new CustomEvent(error.status === 401 ? 'kaeru-auth-required' : 'kaeru-api-error', { detail: error }))
} })
