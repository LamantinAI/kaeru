import { defineConfig } from 'vite'

// In dev, proxy the daemon's HTTP surface so the rooms are always fresh:
// `/v1` is the API (a route per curator verb) and `/graph.json` is the legacy
// alias for `/v1/export`, kept while older snapshots and clients catch up.
// For the talk, `npm run build` bundles public/graph.json (the baked snapshot)
// so the app works with no daemon at all — the API calls simply fail and each
// room falls back to what the snapshot can tell it.
const daemon = () => ({
  target: process.env.KAERU_VIZ_URL || 'http://127.0.0.1:9876',
  changeOrigin: true,
  // When the daemon requires a bearer token, set KAERU_VIZ_TOKEN so the dev
  // proxy authenticates. Unset = no header.
  configure: (proxy) => {
    const t = process.env.KAERU_VIZ_TOKEN
    if (t) proxy.on('proxyReq', (preq) => preq.setHeader('authorization', `Bearer ${t}`))
  },
})

export default defineConfig({
  server: {
    proxy: {
      '/v1': daemon(),
      '/graph.json': daemon(),
    },
  },
  // keep a single three instance shared by the app and three/addons
  // (OrbitControls etc.) — avoids the "multiple instances of three" warning
  resolve: { dedupe: ['three'] },
  build: { outDir: 'dist', chunkSizeWarningLimit: 4000, target: 'esnext' },
  esbuild: { target: 'esnext' },
})
