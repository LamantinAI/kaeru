// Real temporary instance, real JWT cookies and the compiled embedded frontend.
import assert from 'node:assert/strict'
import test from 'node:test'
import { chromium } from '@playwright/test'
import { spawn, spawnSync } from 'node:child_process'
import { createHmac } from 'node:crypto'
import { once } from 'node:events'
import { mkdtemp, rm, writeFile, readFile, mkdir } from 'node:fs/promises'
import { openSync, closeSync } from 'node:fs'
import { tmpdir } from 'node:os'
import { createServer } from 'node:net'
import { resolve, join } from 'node:path'

const binary = process.env.KAERU_MCP_TEST_BINARY || resolve('../target/debug/kaeru-mcp')
const password = 'temporary browser test password'
const mcpToken = 'temporary-mcp-test-token'
async function freePort() {
  const server = createServer()
  server.listen(0, '127.0.0.1'); await once(server, 'listening')
  const port = server.address().port
  await new Promise((resolve) => server.close(resolve)); return port
}
async function stop(child) {
  if (child.exitCode !== null || child.signalCode !== null) return
  child.kill('SIGTERM')
  await Promise.race([once(child, 'exit'), new Promise((resolve) => setTimeout(resolve, 2000))])
  if (child.exitCode === null && child.signalCode === null) {
    child.kill('SIGKILL'); await once(child, 'exit')
  }
}
async function fixture({ authentication = 'enabled', tokenOverride = true } = {}) {
  const directory = await mkdtemp(join(tmpdir(), 'kaeru-browser-'))
  const port = await freePort(), origin = `http://127.0.0.1:${port}`
  const config = join(directory, 'web.toml')
  const env = Object.fromEntries(Object.entries(process.env).filter(([key]) => !key.startsWith('KAERU_')))
  Object.assign(env, { KAERU_VAULT_PATH: join(directory, 'vault'), KAERU_MCP_LISTEN_ADDRESS: '127.0.0.1', KAERU_MCP_LISTEN_PORT: String(port),
    KAERU_MCP_WEB_CONFIG_FILE: config, KAERU_MCP_CLOUDS_FILE: join(directory, 'clouds.toml'),
    KAERU_MCP_AUTH_TOKEN: mcpToken, KAERU_MCP_VIZ_INITIATIVES: '*', KAERU_MCP_VIZ_INCLUDE_LOCAL: '1',
    KAERU_MCP_HYGIENE_ENABLE: '0', KAERU_MCP_UPDATE_CHECK: '0', KAERU_READ_TRACE_ENABLED: 'false' })
  if (authentication === 'default' || !tokenOverride) delete env.KAERU_MCP_AUTH_TOKEN
  await writeFile(env.KAERU_MCP_CLOUDS_FILE, '')
  if (authentication !== 'default') {
    const args = authentication === 'enabled' ? ['--auth', 'enabled', '--password-stdin', '--public-origin', origin] : ['--auth', 'disabled']
    const setup = spawnSync(binary, args, { env, cwd: directory, input: password + '\n', encoding: 'utf8' })
    assert.equal(setup.status, 0, setup.stderr)
  }
  const effectiveToken = !tokenOverride && authentication === 'enabled'
    ? (await readFile(config, 'utf8')).match(/^mcp_token = "([a-f0-9]+)"$/m)[1] : mcpToken
  const logPath = join(directory, 'instance.log'), log = openSync(logPath, 'w')
  const child = spawn(binary, [], { env, cwd: directory, stdio: ['ignore', log, log] }); closeSync(log)
  async function close() { await stop(child); await rm(directory, { recursive: true, force: true }) }
  try {
  let ready = false
  for (let attempt = 0; attempt < 100; attempt++) {
    if (child.exitCode !== null) break
    try { if ((await fetch(origin + '/auth/session')).ok) { ready = true; break } } catch (_) {}
    await new Promise((resolve) => setTimeout(resolve, 100))
  }
  if (!ready) throw new Error(await readFile(logPath, 'utf8'))
  async function api(operation, args) {
    const response = await fetch(`${origin}/v1/${operation}`, { method: 'POST', headers: { 'content-type': 'application/json', authorization: `Bearer ${effectiveToken}` }, body: JSON.stringify(args) })
    const value = await response.json(); assert.ok(response.ok && value.state !== 'rejected', JSON.stringify(value)); return value
  }
  const ids = []
  for (const [name, body, kind, layer] of [
    ['Working memory', 'Observations remain connected to the decisions they support.', 'observation', 'hot'],
    ['Keep the reasoning', 'Preserve the evidence and why a decision was made.', 'decision', 'core'],
    ['One shared contract', 'The same memory operations serve tools and the visualizer.', 'decision', 'warm'],
  ]) ids.push((await api('note', { initiative: 'memory-lab', name, body, kind, options: { layer } })).data.primary.id)
  await api('relate', { initiative: 'memory-lab', action: 'connect', from: ids[2], to: ids[1], type: 'derived_from', weight: 1 })
  await api('relate', { initiative: 'memory-lab', action: 'connect', from: ids[1], to: ids[0], type: 'derived_from', weight: 1 })
  await api('trail', { initiative: 'memory-lab', name: 'Connected reasoning', route: { from: ids[2], to: ids[0] }, summary: 'A shared contract connects the decisions to their supporting observations.' })
  await api('note', { initiative: 'planning', name: 'A quiet workspace', body: 'A small, inspectable instance for connected work.', kind: 'observation' })
  return { origin, directory, config, close, token: effectiveToken }
  } catch (error) { await close(); throw error }
}

async function browser() {
  return chromium.launch({ headless: true, ...(process.env.KAERU_BROWSER_PATH ? { executablePath: process.env.KAERU_BROWSER_PATH } : {}),
    args: ['--no-sandbox', '--use-gl=angle', '--use-angle=swiftshader', '--enable-unsafe-swiftshader'] })
}
async function screenshot(page, name) {
  if (!process.env.KAERU_SCREENSHOT_DIR) return
  await mkdir(process.env.KAERU_SCREENSHOT_DIR, { recursive: true })
  await page.evaluate(() => document.fonts.ready)
  await page.waitForTimeout(350) // Let the existing color transition settle.
  await page.screenshot({ path: join(process.env.KAERU_SCREENSHOT_DIR, name + '.png'), fullPage: true })
}

test('embedded login, live graph, admin, password change and logout', { timeout: 90000 }, async () => {
  const instance = await fixture()
  let engine
  try {
    engine = await browser()
    const context = await engine.newContext({ viewport: { width: 1365, height: 900 }, colorScheme: 'dark', reducedMotion: 'reduce' })
    const page = await context.newPage(), requests = [], errors = []
    page.on('request', (request) => requests.push(new URL(request.url()).pathname))
    page.on('pageerror', (error) => errors.push(error.message))
    page.on('console', (message) => { if (message.type() === 'error' && /Content Security Policy|font-src/.test(message.text())) errors.push(message.text()) })
    await page.goto(instance.origin + '/ui/')
    await page.getByRole('heading', { name: 'Sign in' }).waitFor()
    await screenshot(page, 'login-dark')
    await page.getByRole('button', { name: 'Switch color theme' }).click()
    await screenshot(page, 'login-light')
    await page.getByLabel('Username', { exact: true }).fill('admin')
    await page.getByLabel('Password', { exact: true }).fill('wrong password')
    await page.getByRole('button', { name: /open memory/ }).click()
    await page.getByRole('alert').filter({ hasText: 'Invalid username or password' }).waitFor()
    await page.getByLabel('Password', { exact: true }).fill(password)
    await page.getByRole('button', { name: /open memory/ }).click()
    await page.locator('#graph canvas').waitFor({ timeout: 30000 })
    assert.ok(requests.includes('/v1/context'))
    assert.equal(requests.filter((path) => path === '/v1/read' || path === '/v1/initiatives').length, 0, 'initial graph must not hydrate individual records')
    const held = []
    await page.route('**/v1/read', (route) => { held.push(route) })
    await Promise.all([page.waitForRequest((request) => new URL(request.url()).pathname === '/v1/read'), page.locator('#chainRead').click()])
    await page.locator('#rBack').click()
    const readsAtClose = requests.filter((path) => path === '/v1/read').length
    for (const route of held) await route.abort('aborted').catch(() => {})
    await page.waitForTimeout(100)
    assert.equal(requests.filter((path) => path === '/v1/read').length, readsAtClose, 'closing reader must not restart the cancelled chain')
    await page.unroute('**/v1/read')
    await Promise.all([page.waitForResponse((response) => new URL(response.url()).pathname === '/v1/read'), page.locator('#chainRead').click()])
    assert.ok(requests.includes('/v1/read'), 'opening a trail loads its text lazily')
    await page.locator('#rBack').click()
    await page.locator('#hud').waitFor({ state: 'visible' })
    assert.ok(await page.locator('#chainPick option').allTextContents().then((items) => items.some((text) => text.includes('Connected reasoning'))))
    await screenshot(page, 'galaxy-live')
    const cookie = (await context.cookies()).find((cookie) => cookie.name === 'kaeru_session')
    assert.ok(cookie.httpOnly && cookie.sameSite === 'Strict')
    assert.equal((await page.evaluate(() => [...Object.keys(localStorage), ...Object.keys(sessionStorage)].filter((key) => /token|password|jwt/i.test(key)))).length, 0)
    await page.getByRole('link', { name: /settings/ }).click()
    await page.getByRole('heading', { name: 'Account', exact: true }).waitFor()
    await screenshot(page, 'admin-desktop')
    await page.setViewportSize({ width: 390, height: 844 })
    await screenshot(page, 'admin-mobile')
    assert.ok(await page.evaluate(() => document.documentElement.scrollWidth <= innerWidth + 1), 'mobile admin must not overflow horizontally')
    await page.getByLabel('Username', { exact: true }).fill('operator')
    await page.locator('#account-current').fill(password)
    await page.locator('#account-new').fill('updated browser test password')
    await page.getByRole('button', { name: 'save account' }).click()
    await page.getByRole('heading', { name: 'Sign in' }).waitFor()
    await page.getByLabel('Username', { exact: true }).fill('operator')
    await page.getByLabel('Password', { exact: true }).fill('updated browser test password')
    await page.getByRole('button', { name: /open memory/ }).click()
    await page.locator('#graph canvas').waitFor({ timeout: 30000 })
    await page.locator('[data-logout]').click()
    await page.getByRole('heading', { name: 'Sign in' }).waitFor()
    await page.goto(instance.origin + '/ui/admin')
    await page.getByRole('heading', { name: 'Sign in' }).waitFor()
    assert.equal(requests.filter((path) => path.endsWith('graph.json')).length, 0)
    assert.deepEqual(errors, [])
  } finally { await engine?.close(); await instance.close() }
})

test('a correctly signed but expired JWT cannot reveal the graph', { timeout: 30000 }, async () => {
  const instance = await fixture()
  let engine
  try {
    engine = await browser()
    const text = await readFile(instance.config, 'utf8')
    const secret = text.match(/^jwt_secret = "([a-f0-9]+)"$/m)[1]
    const now = Math.floor(Date.now() / 1000)
    const enc = (value) => Buffer.from(JSON.stringify(value)).toString('base64url')
    const value = enc({ alg: 'HS256', typ: 'JWT' }) + '.' + enc({ sub: 'admin', exp: now - 1, iat: now - 3601, nbf: now - 3601, iss: 'kaeru-mcp', aud: 'kaeru-web', revision: 1, csrf: 'a'.repeat(64) })
    const token = value + '.' + createHmac('sha256', secret).update(value).digest('base64url')
    const context = await engine.newContext()
    await context.addCookies([{ name: 'kaeru_session', value: token, url: instance.origin, httpOnly: true, sameSite: 'Strict', expires: now + 3600 }])
    const page = await context.newPage(), requests = []
    page.on('request', (request) => requests.push(new URL(request.url()).pathname))
    await page.goto(instance.origin + '/ui/')
    await page.getByRole('heading', { name: 'Sign in' }).waitFor()
    assert.equal(requests.filter((path) => path.startsWith('/v1/') || path.endsWith('graph.json')).length, 0)
  } finally { await engine?.close(); await instance.close() }
})

test('headless startup does not require a home or browser configuration directory', { timeout: 20000 }, async () => {
  const directory = await mkdtemp(join(tmpdir(), 'kaeru-headless-'))
  const port = await freePort()
  const env = Object.fromEntries(Object.entries(process.env).filter(([key]) =>
    !key.startsWith('KAERU_') && !['HOME', 'XDG_CONFIG_HOME', 'APPDATA', 'USERPROFILE'].includes(key)))
  Object.assign(env, { KAERU_VAULT_PATH: join(directory, 'vault'), KAERU_MCP_UI_ENABLE: '0',
    KAERU_MCP_CLOUDS_FILE: join(directory, 'clouds.toml'), KAERU_MCP_AUTH_TOKEN: mcpToken,
    KAERU_MCP_LISTEN_ADDRESS: '127.0.0.1', KAERU_MCP_LISTEN_PORT: String(port),
    KAERU_MCP_HYGIENE_ENABLE: '0', KAERU_MCP_UPDATE_CHECK: '0', KAERU_READ_TRACE_ENABLED: 'false' })
  await writeFile(env.KAERU_MCP_CLOUDS_FILE, '')
  const logPath = join(directory, 'instance.log'), log = openSync(logPath, 'w')
  const child = spawn(binary, [], { env, cwd: directory, stdio: ['ignore', log, log] }); closeSync(log)
  try {
    let ready = false
    for (let attempt = 0; attempt < 100; attempt++) {
      if (child.exitCode !== null) break
      try {
        const response = await fetch(`http://127.0.0.1:${port}/mcp`, { signal: AbortSignal.timeout(1000) })
        if (response.status === 401) { ready = true; break }
      } catch (_) {}
      await new Promise((resolve) => setTimeout(resolve, 100))
    }
    assert.ok(ready, await readFile(logPath, 'utf8'))
    assert.equal((await fetch(`http://127.0.0.1:${port}/ui/`, { headers: { authorization: `Bearer ${mcpToken}` } })).status, 404)
  } finally { await stop(child); await rm(directory, { recursive: true, force: true }) }
})

async function initialize(origin, token) {
  return fetch(origin + '/mcp', { method: 'POST', headers: {
    'content-type': 'application/json', accept: 'application/json, text/event-stream',
    ...(token ? { authorization: `Bearer ${token}` } : {}),
  }, body: JSON.stringify({ jsonrpc: '2.0', id: 1, method: 'initialize', params: {
    protocolVersion: '2025-03-26', capabilities: {}, clientInfo: { name: 'auth-check', version: '1' },
  } }) })
}

test('anonymous UI opens the graph and APIs without login, including with leftover tokens', { timeout: 90000 }, async () => {
  let engine
  try {
    engine = await browser()
    for (const authentication of ['disabled', 'default']) {
      const instance = await fixture({ authentication })
      try {
        const page = await engine.newPage()
        await page.goto(instance.origin + '/ui/')
        await page.locator('#graph canvas').waitFor({ timeout: 20000 })
        assert.equal(await page.locator('#login-form').count(), 0)
        assert.equal(await page.locator('.account-nav').count(), 0)
        const session = await (await fetch(instance.origin + '/auth/session')).json()
        assert.equal(session.auth_enabled, false)
        assert.equal(session.authenticated, false)
        assert.equal(session.csrf_token, null)
        assert.equal((await fetch(instance.origin + '/v1/initiatives', { method: 'POST', headers: {
          'content-type': 'application/json', authorization: 'Bearer obsolete', cookie: 'kaeru_session=expired',
        }, body: '{}' })).status, 200)
        const init = await initialize(instance.origin)
        assert.equal(init.status, 200); await init.text()
        assert.equal((await fetch(instance.origin + '/admin/settings')).status, 404)
        assert.equal((await fetch(instance.origin + '/auth/login', { method: 'POST', headers: { 'content-type': 'application/json' }, body: '{}' })).status, 404)
        for (const path of ['/ui/login', '/ui/admin']) {
          await page.goto(instance.origin + path)
          await page.locator('#graph canvas').waitFor({ timeout: 20000 })
          assert.equal(new URL(page.url()).pathname, '/ui/')
        }
        await page.close()
      } finally { await instance.close() }
    }
  } finally { await engine?.close() }
})

test('configured MCP token protects requests without a token environment override', { timeout: 30000 }, async () => {
  const instance = await fixture({ tokenOverride: false })
  try {
    assert.equal((await initialize(instance.origin)).status, 401)
    assert.equal((await initialize(instance.origin, 'wrong')).status, 401)
    const response = await initialize(instance.origin, instance.token)
    assert.equal(response.status, 200); await response.text()
    assert.equal((await fetch(instance.origin + '/v1/initiatives', { method: 'POST', headers: { 'content-type': 'application/json' }, body: '{}' })).status, 401)
  } finally { await instance.close() }
})

test('local authentication choice persists and can be disabled and enabled again', { timeout: 30000 }, async () => {
  const directory = await mkdtemp(join(tmpdir(), 'kaeru-auth-choice-'))
  const config = join(directory, 'web.toml')
  const env = Object.fromEntries(Object.entries(process.env).filter(([key]) => !key.startsWith('KAERU_')))
  env.KAERU_MCP_WEB_CONFIG_FILE = config
  env.KAERU_MCP_CLOUDS_FILE = join(directory, 'clouds.toml')
  await writeFile(env.KAERU_MCP_CLOUDS_FILE, '')
  const run = (args, input = '') => spawnSync(binary, args, { env, cwd: directory, input, encoding: 'utf8' })
  try {
    assert.equal(run(['--auth-status']).stdout.trim(), 'disabled')
    assert.equal(run(['--auth', 'disabled']).status, 0)
    assert.equal(run(['--auth-status']).stdout.trim(), 'disabled')
    assert.equal((await readFile(config, 'utf8')).includes('password_hash'), false)
    const setup = run(['--auth', 'enabled', '--password-stdin'], password + '\n')
    assert.equal(setup.status, 0, setup.stderr)
    const first = await readFile(config, 'utf8')
    assert.match(first, /^mcp_token = "[a-f0-9]{64}"$/m)
    assert.equal(run(['--auth-status']).stdout.trim(), 'enabled')
    const repeated = run(['--auth', 'enabled', '--password-stdin'], 'not a password reset\n')
    assert.equal(repeated.status, 0, repeated.stderr)
    assert.equal(await readFile(config, 'utf8'), first)
    assert.equal(run(['--auth', 'disabled']).status, 0)
    assert.equal((await readFile(config, 'utf8')).includes('jwt_secret'), false)
    assert.equal(run(['--auth', 'enabled', '--password-stdin'], password + '\n').status, 0)
    assert.notEqual((await readFile(config, 'utf8')).match(/^jwt_secret = "([^"]+)"/m)[1], first.match(/^jwt_secret = "([^"]+)"/m)[1])
    await writeFile(config, 'enabled = true\n')
    assert.notEqual(run(['--auth-status']).status, 0, 'incomplete protected configuration must fail closed')
    assert.equal((setup.stdout + setup.stderr).includes(password), false)
  } finally { await rm(directory, { recursive: true, force: true }) }
})
