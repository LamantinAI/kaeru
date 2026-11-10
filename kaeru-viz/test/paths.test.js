import test from 'node:test'
import assert from 'node:assert/strict'
import { createPaths } from '../src/paths.js'
import { createAuthClient } from '../src/auth.js'
import { createClient } from '../src/api.js'

test('legacy and nested mounts produce explicit same-origin paths', () => {
  const legacy = createPaths()
  assert.equal(legacy.page('/login?notice=expired'), '/ui/login?notice=expired')
  assert.equal(legacy.request('/auth/session'), '/auth/session')
  const nested = createPaths({ uiBase: '/tools/memory', apiBase: '/tools/memory' })
  assert.equal(nested.page('/'), '/tools/memory/')
  assert.equal(nested.request('/v1/read'), '/tools/memory/v1/read')
  assert.ok(nested.isPage('/tools/memory/admin', '/admin'))
  for (const base of ['//foreign.test', '/a/../b', '/a%2fb', '/a?x', '/a//b']) assert.throws(() => createPaths({ uiBase: base }))
})
test('prefixed session bootstrap and memory reads stay inside the supplied mount', async () => {
  const calls = []
  const fetcher = async (path, options) => {
    calls.push({ path, options })
    return { ok: true, status: 200, json: async () => ({ state: 'ok', csrf_token: 'synthetic', data: {} }) }
  }
  await createAuthClient({ baseUrl: '/tools/memory', fetcher }).session()
  await createClient({ baseUrl: '/tools/memory', fetcher }).call('read', { initiative: 'synthetic', node: 'synthetic' })
  assert.equal(calls[0].path, '/tools/memory/auth/session')
  assert.equal(calls[0].options.method, 'POST')
  assert.equal(calls[1].path, '/tools/memory/v1/read')
  assert.ok(calls.every(({ options }) => options.credentials === 'same-origin' && !('authorization' in options.headers)))
})
