import { sessionHeaders, setSession } from './session.js'

export function createAuthClient({ fetcher = fetch } = {}) {
  async function request(path, { method = 'GET', body } = {}) {
    const response = await fetcher(path, {
      method, credentials: 'same-origin', headers: { 'content-type': 'application/json', ...sessionHeaders() },
      ...(body === undefined ? {} : { body: JSON.stringify(body) }),
    })
    const value = response.status === 204 ? {} : await response.json().catch(() => ({}))
    if (!response.ok) {
      const error = new Error(value.error || `Request failed (${response.status})`)
      error.status = response.status
      throw error
    }
    return value
  }
  return {
    request,
    async session() { const value = await request('/auth/session'); setSession(value); return value },
    async login(username, password) { const value = await request('/auth/login', { method: 'POST', body: { username, password } }); setSession(value); return value },
    async logout() { await request('/auth/logout', { method: 'POST' }); setSession(null) },
  }
}
export const auth = createAuthClient()
