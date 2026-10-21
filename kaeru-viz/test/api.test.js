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

test('the graph loads one compact read and never hydrates every record', async () => {
  const calls = []
  const client = createClient({ fetcher: async (url, request) => {
    const args = JSON.parse(request.body); calls.push([url, args])
    assert.equal(url, '/v1/context'); assert.equal(args.view, 'graph'); assert.deepEqual(args.initiatives, ['work'])
    return reply({ coverage: { initiatives: ['work'] }, data: { graph: { snapshot_id: 'one', nodes: [{ id: 'a', node_type: 'episode', name: 'A', tags: [], initiatives: ['work'] }], edges: [{ from: 'a', to: 'absent', edge_type: 'causal', weight: 1 }], members: [] } } })
  } })
  const graph = await client.graph(['work'])
  assert.equal(calls.length, 1)
  assert.deepEqual(graph.initiatives.map((s) => s.name), ['work'])
  assert.deepEqual(graph.edges, [])
  assert.equal(graph.nodes[0].degree, 0)
  assert.equal(graph.nodes[0].body_loaded, false)
  assert.equal(graph.nodes[0].body, undefined)
})

test('compact graph pages assemble ordered chains and abort propagates without an error banner', async () => {
  const abort = new AbortController(), calls = []
  const client = createClient({ fetcher: async (url, request) => {
    assert.equal(request.signal, abort.signal)
    const args = JSON.parse(request.body); calls.push(args)
    const graph = { snapshot_id: 'one', edges: [], members: [], nodes: [] }
    if (!args.cursor) graph.nodes = [{ id: 'a', name: 'A', initiatives: ['work'] }, { id: 'b', name: 'B', initiatives: ['work'] }, { id: 'chain', name: 'Order', initiatives: ['work'] }]
    else graph.members = [{ chain_id: 'chain', node_id: 'b', position: 1 }, { chain_id: 'chain', node_id: 'a', position: 0 }]
    return reply({ coverage: { initiatives: ['work'] }, data: { graph }, ...(!args.cursor ? { continuation: { next_arguments: { ...args, cursor: 'next' } } } : {}) })
  } })
  const graph = await client.graph(undefined, { signal: abort.signal })
  assert.deepEqual(graph.chains[0].members, ['a', 'b']); assert.equal(calls.length, 2)
  let errors = 0
  const cancelled = createClient({ onError: () => errors++, fetcher: async () => { throw new DOMException('Cancelled', 'AbortError') } })
  await assert.rejects(cancelled.graph(), { name: 'AbortError' }); assert.equal(errors, 0)
})

test('aborting response-body delivery remains an AbortError', async () => {
  let errors = 0
  const client = createClient({ onError: () => errors++, fetcher: async () => ({ ok: true, status: 200, json: async () => { throw new DOMException('Cancelled body', 'AbortError') } }) })
  await assert.rejects(client.graph(), { name: 'AbortError' })
  assert.equal(errors, 0)
})
