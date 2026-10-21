import assert from 'node:assert/strict'
import test from 'node:test'
import { createAuthClient } from '../src/auth.js'
import { setSession, sessionHeaders } from '../src/session.js'
import { createClient } from '../src/api.js'

const reply = (body, status = 200) => ({ ok: status >= 200 && status < 300, status, json: async () => body })

test('browser sign-in uses same-origin cookies and subsequent requests carry CSRF', async () => {
  setSession(null)
  const calls = []
  const client = createAuthClient({ fetcher: async (path, request) => {
    calls.push({ path, ...request })
    if (path === '/auth/login') return reply({ authenticated: true, username: 'operator', csrf_token: 'nonce' })
    return reply({}, 204)
  } })
  await client.login('operator', 'temporary test password')
  assert.equal(calls[0].credentials, 'same-origin')
  assert.deepEqual(JSON.parse(calls[0].body), { username: 'operator', password: 'temporary test password' })
  assert.deepEqual(sessionHeaders(), { 'x-kaeru-csrf': 'nonce' })
  await client.logout()
  assert.equal(calls[1].headers['x-kaeru-csrf'], 'nonce')
  assert.equal(calls[1].headers.authorization, undefined)
  assert.deepEqual(sessionHeaders(), {})
})

test('failed sign-in is reported without establishing a browser session', async () => {
  setSession(null)
  const client = createAuthClient({ fetcher: async () => reply({ error: 'Invalid username or password' }, 401) })
  await assert.rejects(client.login('operator', 'wrong'), (error) => error.status === 401 && error.message === 'Invalid username or password')
  assert.deepEqual(sessionHeaders(), {})
})

test('API failure stays a failure and never requests a graph snapshot', async () => {
  const paths = [], errors = []
  const client = createClient({ fetcher: async (path) => { paths.push(path); return reply({ error: 'Sign in' }, 401) }, onError: (error) => errors.push(error) })
  await assert.rejects(client.graph(), (error) => error.status === 401)
  assert.deepEqual(paths, ['/v1/context'])
  assert.equal(errors.length, 1)
})
