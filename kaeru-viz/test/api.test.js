import assert from 'node:assert/strict'
import test from 'node:test'
import { createClient, registerGraph } from '../src/api.js'
const reply = (value, status = 200) => ({ ok: status === 200, status, json: async () => value })

test('a stale continuation restarts the read instead of mixing revisions', async () => {
  let initial = 0
  const client = createClient({ fetcher: async (_url, request) => {
    const args = JSON.parse(request.body)
    if (args.cursor) return reply({ state: 'rejected', issues: [{ code: 'CURSOR_STALE' }] }, 400)
    initial++
    return initial === 1
      ? reply({ state: 'ok', data: { version: 'old' }, continuation: { next_arguments: { cursor: 'old' } } })
      : reply({ state: 'ok', data: { version: 'new' } })
  } })
  const pages = await client.pages('read', { initiative: 'work', node: 'a' })
  assert.deepEqual(pages.map((p) => p.data.version), ['new'])
})

test('record bodies assemble all pages with the visible node scope', async () => {
  registerGraph({ nodes: [{ id: 'source', initiatives: ['work'] }] })
  const client = createClient({ fetcher: async (url, request) => {
    assert.equal(url, '/v1/read')
    const args = JSON.parse(request.body)
    assert.equal(args.initiative, 'work')
    return args.cursor
      ? reply({ state: 'ok', data: { node: { body: '日本語 🐸' } } })
      : reply({ state: 'ok', data: { node: { body: 'Exact ' } }, continuation: { next_arguments: { initiative: 'work', node: 'source', cursor: 'next' } } })
  } })
  assert.equal(await client.record('source'), 'Exact 日本語 🐸')
})

test('the live graph keeps requested scopes and drops edges to absent nodes', async () => {
  const client = createClient({ fetcher: async (url, request) => {
    const args = JSON.parse(request.body)
    if (url.endsWith('/initiatives')) return reply({ data: { initiatives: ['work', 'other'] } })
    assert.equal(args.initiative, 'work')
    if (url.endsWith('/context')) return reply({ data: { sections: [{ records: [{ id: 'a', node_type: 'episode', name: 'A', tags: [] }] }] } })
    return reply({ data: { node: { body: 'Visible.' }, relations: [{ from: 'a', to: 'absent', edge_type: 'causal', weight: 1 }], reasoning: [] } })
  } })
  const graph = await client.graph(['work'])
  assert.deepEqual(graph.initiatives.map((s) => s.name), ['work'])
  assert.deepEqual(graph.edges, [])
  assert.equal(graph.nodes[0].degree, 0)
})
