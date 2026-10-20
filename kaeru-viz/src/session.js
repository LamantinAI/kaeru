// Browser state contains no password or bearer token. JWT is an HttpOnly cookie.
let current = null
export function setSession(value) { current = value?.authenticated ? value : null }
export function sessionHeaders() { return current?.csrf_token ? { 'x-kaeru-csrf': current.csrf_token } : {} }
export function currentSession() { return current }
