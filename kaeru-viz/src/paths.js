// Runtime HTML metadata is the sole source of browser mount configuration.
function path(value, fallback) {
  value ||= fallback
  if (value && (!value.startsWith('/') || value.startsWith('//') || /[^A-Za-z0-9/_.-]/.test(value) || value.slice(1).split('/').some((s) => !s || s === '.' || s === '..'))) throw new Error('Invalid browser mount metadata')
  return value
}
export function createPaths({ uiBase = '/ui', apiBase = '' } = {}) {
  const ui = path(uiBase, '/ui'), api = path(apiBase, '')
  return Object.freeze({ uiBase: ui, apiBase: api,
    page: (suffix = '/') => `${ui}${suffix}`,
    request: (suffix) => `${api}${suffix}`,
    isPage: (pathname, suffix) => pathname === `${ui}${suffix}`,
  })
}
const metadata = typeof document === 'undefined' ? {} : document.documentElement.dataset
export const paths = createPaths({ uiBase: metadata.kaeruUiBase || '/ui', apiBase: metadata.kaeruApiBase || '' })
