import { mountGalaxy, mountStarfield } from './login-galaxy.js'
import { auth } from './auth.js'
import { memory, registerGraph } from './api.js'
import './account.css'

const esc = (value) => String(value ?? '').replace(/[&<>"']/g, (c) => ({ '&': '&amp;', '<': '&lt;', '>': '&gt;', '"': '&quot;', "'": '&#39;' }[c]))
const go = (path) => location.replace(path)
// A message lands in the nearest notice: a form's own, so an error sits beside
// the fields it is about, or else the page's. Returns false where there is none.
const notice = (text, error = false, scope = document) => {
  const target = scope.querySelector('[data-notice]')
  if (target) { target.textContent = text; target.classList.toggle('error', error); target.hidden = !text }
  return Boolean(target)
}
// Every way a session ends unasked (expiry, a revoked token) arrives at the
// sign-in page saying so, rather than at a blank form.
const expired = () => go('/ui/login?notice=expired')
function theme() {
  let value = 'auto'
  try { value = localStorage.getItem('kaeru-viz-theme') || 'auto' } catch (_) {}
  document.documentElement.dataset.theme = value === 'auto' ? (matchMedia('(prefers-color-scheme: light)').matches ? 'light' : 'dark') : value
}
function wireTheme() {
  document.querySelector('[data-theme-toggle]')?.addEventListener('click', () => {
    const value = document.documentElement.dataset.theme === 'light' ? 'dark' : 'light'
    // The page background transitions while the color tokens snap, so a
    // switch shows a frame of low-contrast text. Switch with transitions
    // off, then restore them after the next frame.
    const root = document.documentElement
    root.classList.add('theme-switching')
    root.dataset.theme = value
    void root.offsetHeight
    requestAnimationFrame(() => root.classList.remove('theme-switching'))
    try { localStorage.setItem('kaeru-viz-theme', value) } catch (_) {}
  })
}
// The project turns slowly while the page is open; under reduced motion it
// is drawn once and stays still.
function wireGalaxy() {
  const frame = document.querySelector('.galaxy-frame')
  if (!frame || getComputedStyle(frame).display === 'none') return
  const galaxy = mountGalaxy(frame, { onTurn: starfield?.rotate })
  starfield?.place() // the field turns about the project's axis, now that it is laid out
  if (!matchMedia('(prefers-reduced-motion: reduce)').matches) galaxy.play()
}
const mark = `<a class="account-brand" href="/ui/" aria-label="Kaeru memory"><span class="seal">蛙</span><span>kaeru<small>Cognitive memory for agents</small></span></a>`
const footer = `<footer class="account-footer"><a href="https://lamantin-ai.com" target="_blank" rel="noopener">Lamantin AI</a><span>connected memory</span></footer>`
let starfield = null
function shell(content, { admin = false, username = '' } = {}) {
  document.body.className = 'account-page'
  document.body.innerHTML = `<div class="account-shell"><header class="account-header">${mark}<nav>
    ${admin ? `<a class="btn" href="/ui/">← memory</a><span class="account-user">${esc(username)}</span>` : ''}
    <button type="button" class="btn" data-theme-toggle aria-label="Switch color theme">◐</button>
    ${admin ? '<button type="button" class="btn" data-logout>sign out all sessions</button>' : ''}
    </nav></header>${content}${footer}</div>`
  wireTheme()
  starfield = mountStarfield(document.body)
  document.querySelector('[data-logout]')?.addEventListener('click', logout)
}
async function logout() {
  try { await auth.logout(); go('/ui/login?notice=signed-out') } catch (error) {
    // the galaxy has no notice of its own; its live error bar carries the failure
    if (error.status === 401) expired()
    else if (!notice(error.message, true)) window.dispatchEvent(new CustomEvent('kaeru-api-error', { detail: error }))
  }
}
function login(configured) {
  document.title = 'Sign in — kaeru'
  shell(`<main class="access-layout"><section class="access-story"><p class="account-eyebrow">a working memory</p>
    <h1>Keep the thread.</h1><p class="account-lead">Decisions, connections and unfinished thoughts.<br>Ready to return to.</p>
    <div class="galaxy-frame"></div>
    </section><section class="access-card" aria-labelledby="access-title"><p class="account-eyebrow">${configured ? 'welcome back' : 'first connection'}</p>
    <h2 id="access-title">${configured ? 'Sign in' : 'Set up access'}</h2>
    ${configured ? '' : '<p class="account-hint">Create an account on the machine running kaeru, then restart it.</p>'}
    ${configured ? `<form id="login-form"><label for="username">Username</label><input id="username" name="username" autocomplete="username" required maxlength="128" />
      <label for="password">Password</label><input id="password" name="password" type="password" autocomplete="current-password" required maxlength="1024" />
      <p class="account-notice" data-notice role="alert" hidden></p><button class="btn go account-submit" type="submit">open memory</button></form>`
      : `<code class="setup-command">kaeru-mcp --init-web-auth</code><p class="account-notice" data-notice role="status" hidden></p><button class="btn account-submit" data-retry>check again →</button>`}
    </section></main>`)
  wireGalaxy()
  document.querySelector('[data-retry]')?.addEventListener('click', () => location.reload())
  const known = { 'signed-out': 'All browser sessions have been signed out.', 'account-updated': 'Account updated. Sign in with your current credentials.', 'settings-updated': 'Session settings saved. Sign in again.', expired: 'Your session ended. Sign in again.' }
  notice(known[new URLSearchParams(location.search).get('notice')] || '')
  document.querySelector('#login-form')?.addEventListener('submit', async (event) => {
    event.preventDefault()
    const form = event.currentTarget, button = form.querySelector('button')
    button.disabled = true; notice('Signing in…')
    try { await auth.login(form.username.value, form.password.value); form.password.value = ''; go('/ui/') }
    catch (error) {
      // a forgotten password has no reset link to follow; say where the reset lives
      notice(error.status === 401 ? `${error.message}. Forgot the password? Run kaeru-mcp --reset-web-password on the machine running kaeru.` : error.message, true)
      form.password.value = ''; form.password.focus()
    }
    finally { button.disabled = false }
  })
}
function problem(message, { empty = false, showSettings = false } = {}) {
  shell(`<main class="account-status"><p class="account-eyebrow">${empty ? 'a fresh page' : 'connection paused'}</p><h1>${empty ? 'No visible memories yet.' : 'Memory is unavailable.'}</h1>
    <p class="account-lead">${esc(message)}</p><p class="account-notice" data-notice hidden></p><div class="account-actions"><button class="btn go" data-retry>try again</button>${showSettings ? '<a class="btn" href="/ui/admin">settings</a>' : ''}</div></main>`)
  document.querySelector('[data-retry]').addEventListener('click', () => location.reload())
}
async function admin(session) {
  const cfg = await auth.request('/admin/settings')
  document.title = 'Settings — kaeru'
  // `*` is the configuration's glob for every initiative; the page says it in words
  const chips = (items) => items.length ? items.map((value) => `<span class="account-chip">${value === '*' ? 'all initiatives' : esc(value)}</span>`).join('') : '<span class="account-muted">none configured</span>'
  // The space before <br> keeps the words apart where narrow screens hide the break.
  shell(`<main class="admin-layout"><aside class="admin-intro"><p class="account-eyebrow">settings</p><h1>A place <br>of your own.</h1>
    <p class="account-lead">Manage access.<br>Memory stays in the same place.</p><p class="account-version">kaeru ${esc(cfg.version)}</p></aside>
    <div class="admin-panels"><p class="account-notice" data-notice role="alert" hidden></p>
    <section class="admin-card" id="account"><div class="admin-card-heading"><span>01</span><h2>Account</h2><span class="account-chip">local account</span></div><p class="account-hint">Changes sign out every browser session. Confirm them with your current password.</p>
    <form id="account-form"><label for="account-username">Username</label><input id="account-username" name="username" value="${esc(cfg.username)}" autocomplete="username" required maxlength="128" />
    <label for="account-current">Current password</label><input id="account-current" name="current_password" type="password" autocomplete="current-password" required />
    <label for="account-new">New password <small>optional</small></label><input id="account-new" name="new_password" type="password" autocomplete="new-password" maxlength="1024" />
    <p class="account-notice" data-notice role="alert" hidden></p><button type="submit" class="btn go">save account</button></form></section>
    <section class="admin-card" id="sessions"><div class="admin-card-heading"><span>02</span><h2>Sessions</h2></div><p class="account-hint">Choose how long a sign-in lasts. Saving this setting signs out all sessions.</p>
    <form id="session-form"><label for="session-ttl">Lifetime in minutes</label><input id="session-ttl" name="ttl" type="number" min="5" max="1440" step="any" value="${cfg.ttl_seconds / 60}" required />
    <label for="session-current">Current password</label><input id="session-current" name="current_password" type="password" autocomplete="current-password" required />
    <p class="account-notice" data-notice role="alert" hidden></p><button type="submit" class="btn go">save session settings</button></form></section>
    <section class="admin-card" id="instance"><div class="admin-card-heading"><span>03</span><h2>Access</h2><span class="account-chip">read only</span></div>
    <dl class="instance-facts"><dt>Address</dt><dd>${esc(cfg.public_origin || location.origin)}</dd><dt>Visible initiatives</dt><dd>${chips(cfg.allow)}</dd><dt>Excluded initiatives</dt><dd>${chips(cfg.deny)}</dd><dt>Local records</dt><dd>${cfg.include_local ? 'included' : 'private'}</dd><dt>MCP token</dt><dd>${cfg.mcp_static_auth ? 'configured' : 'required; not configured'}</dd><dt>Connected clouds</dt><dd>${chips(cfg.clouds)}</dd></dl>
    <p class="account-hint">Memory permissions are managed separately.</p></section></div></main>`, { admin: true, username: session.username })
  const submit = (selector, action) => document.querySelector(selector).addEventListener('submit', async (event) => {
    event.preventDefault(); const form = event.currentTarget, button = form.querySelector('button'), current = form.current_password
    button.disabled = true; current.removeAttribute('aria-invalid'); notice('Saving…', false, form)
    try { await action(form); form.reset() } catch (error) {
      if (error.status === 401) expired()
      else {
        notice(error.message, true, form)
        // the one field the server can reject by itself: mark it and go back to it
        if (/current password/i.test(error.message)) { current.setAttribute('aria-invalid', 'true'); current.select(); current.focus() }
      }
    }
    finally { button.disabled = false }
  })
  submit('#account-form', async (form) => { await auth.request('/admin/account', { method: 'PATCH', body: { username: form.username.value, current_password: form.current_password.value, new_password: form.new_password.value || null, revision: cfg.revision } }); go('/ui/login?notice=account-updated') })
  submit('#session-form', async (form) => { await auth.request('/admin/settings', { method: 'PATCH', body: { current_password: form.current_password.value, ttl_seconds: Math.round(Number(form.ttl.value) * 60), revision: cfg.revision } }); go('/ui/login?notice=settings-updated') })
}

theme()
window.addEventListener('pageshow', (event) => { if (event.persisted) location.reload() })
window.addEventListener('kaeru-auth-required', expired)
window.addEventListener('kaeru-api-error', (event) => {
  let target = document.getElementById('live-error')
  if (!target) { target = document.createElement('div'); target.id = 'live-error'; target.className = 'live-error'; target.setAttribute('role', 'alert'); document.body.append(target) }
  target.textContent = `${event.detail.message} — reload to retry.`
})
try {
  const session = await auth.session()
  if (session.authenticated) setTimeout(expired, Math.max(0, session.expires_at * 1000 - Date.now()))
  if (session.auth_enabled && !session.authenticated) login(session.configured)
  else if (location.pathname === '/ui/admin' && session.auth_enabled) await admin(session)
  else if (location.pathname === '/ui/login' || location.pathname === '/ui/admin') go('/ui/')
  else {
    const graphAbort = new AbortController()
    window.addEventListener('pagehide', () => graphAbort.abort(), { once: true })
    const graph = await memory.graph(undefined, { signal: graphAbort.signal })
    if (!graph.nodes.length) problem('There are no visible records under the current access settings.', { empty: true, showSettings: session.auth_enabled })
    else {
      registerGraph(graph)
      if (session.auth_enabled) {
        const nav = document.createElement('nav'); nav.className = 'account-nav'
        // Signing out ends every session (the server revokes by account revision),
        // so the consequence rides along as a hint; the settings page names it in full.
        nav.innerHTML = `<span>${esc(session.username)}</span><a class="btn" href="/ui/admin">settings</a><button class="btn" type="button" data-logout title="Signs out every browser using this memory">sign out</button>`
        document.getElementById('hud').append(nav); nav.querySelector('[data-logout]').addEventListener('click', logout)
      }
      await import('./main.js')
    }
  }
} catch (error) {
  if (error.name === 'AbortError') { /* navigation cancelled the graph load */ }
  else if (error.status === 401) expired()
  else problem(error.status === 403 ? 'Your account cannot access the requested memory.' : error.message)
}
