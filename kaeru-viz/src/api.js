// The visualizer consumes the same named JSON operations as agents.
const scopesByNode = new Map()
let snapshot = null

export function registerGraph(graph) {
  snapshot = graph
  for (const node of graph.nodes || []) if (node.initiatives?.length) scopesByNode.set(node.id, node.initiatives[0])
}
export function scopeFor(id) { return scopesByNode.get(id) }
export function snapshotBody(id) { return snapshot?.nodes.find((n) => n.id === id)?.body }

export function createClient({ baseUrl = '', headers = {}, fetcher = fetch } = {}) {
  async function call(operation, arguments_) {
    const response = await fetcher(`${baseUrl}/v1/${operation}`, {
      method: 'POST', headers: { 'content-type': 'application/json', ...headers },
      body: JSON.stringify(arguments_),
    })
    const value = await response.json()
    if (!response.ok || value.state === 'rejected' || value.state === 'partial') {
      const error = new Error(value.issues?.[0]?.message || `Memory HTTP ${response.status}`)
      error.code = value.issues?.[0]?.code
      throw error
    }
    return value
  }
  // Gather one coherent read. Restart a stale snapshot a bounded number of
  // times, rather than mixing revisions into a seemingly complete graph.
  async function pages(operation, arguments_, stop = () => false) {
    for (let attempt = 0; attempt < 3; attempt++) {
      const out = []
      let next = arguments_
      try {
        for (let page = 0; next && page < 10000; page++) {
          const value = await call(operation, next)
          out.push(value)
          if (stop(value)) return out
          next = value.continuation?.next_arguments
        }
        if (next) throw new Error('Memory pagination limit exceeded')
        return out
      } catch (error) { if (error.code !== 'CURSOR_STALE' || attempt === 2) throw error }
    }
  }
  async function record(id) {
    const initiative = scopeFor(id)
    if (!initiative) throw new Error('No visible initiative for this record')
    const responses = await pages('read', { initiative, node: id, budget_chars: 64000 })
    return responses.map((value) => value.data.node.body || '').join('')
  }
  async function graph(selectedScopes) {
    const scopePages = await pages('initiatives', {})
    const available = scopePages.flatMap((page) => page.data.initiatives)
    const scopes = selectedScopes?.length ? available.filter((s) => selectedScopes.includes(s)) : available
    const nodes = new Map(), edges = new Map(), chains = new Map()
    for (const initiative of scopes) {
      for (const page of await pages('context', { initiative, view: 'map', budget_chars: 64000 })) {
        for (const section of page.data.sections) for (const node of section.records) {
          const previous = nodes.get(node.id)
          nodes.set(node.id, { ...node, type: node.node_type, initiatives: [...new Set([...(previous?.initiatives || []), initiative])],
            created_secs: (node.created_at || node.recorded_at) ? Date.parse(node.created_at || node.recorded_at) / 1000 : null, degree: 0, redacted: node.body === '[redacted]' })
          scopesByNode.set(node.id, initiative)
        }
      }
    }
    for (const node of nodes.values()) {
      // Graph sections precede body chunks. Once body delivery starts, the
      // graph is complete and loading the remainder of a source is unnecessary.
      const responses = await pages('read', { initiative: scopeFor(node.id), node: node.id, include: ['relations', 'reasoning'], budget_chars: 64000 },
        (page) => Boolean(page.data.node.body?.length))
      for (const page of responses) {
        for (const edge of page.data.relations) if (nodes.has(edge.from) && nodes.has(edge.to)) {
          const key = `${edge.from}|${edge.to}|${edge.edge_type}`
          edges.set(key, { src: edge.from, dst: edge.to, type: edge.edge_type, weight: edge.weight, dst_store: edge.dst_store })
        }
        if (node.type === 'chain') for (const trail of page.data.reasoning) if (trail.id === node.id) {
          chains.set(trail.id, { id: trail.id, name: trail.name, members: trail.members.filter((id) => nodes.has(id)) })
        }
      }
    }
    for (const edge of edges.values()) { nodes.get(edge.src).degree++; nodes.get(edge.dst).degree++ }
    const times = [...nodes.values()].map((n) => n.created_secs).filter(Number.isFinite)
    return { nodes: [...nodes.values()], edges: [...edges.values()], chains: [...chains.values()], project_links: [],
      initiatives: scopes.map((name) => ({ name, node_count: [...nodes.values()].filter((n) => n.initiatives.includes(name)).length })),
      meta: { node_count: nodes.size, edge_count: edges.size, chain_count: chains.size, initiative_count: scopes.length,
        redacted_count: [...nodes.values()].filter((n) => n.redacted).length, earliest_secs: times.length ? Math.min(...times) : null, latest_secs: times.length ? Math.max(...times) : null } }
  }
  return { call, pages, record, graph }
}
export const memory = createClient()
