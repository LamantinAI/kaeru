import { defineConfig } from 'vite'

// Keep browser auth, administration and memory requests on one origin.
const daemon = () => ({
  target: process.env.KAERU_VIZ_URL || 'http://127.0.0.1:9876',
  changeOrigin: false,
  // When the daemon requires a bearer token, set KAERU_VIZ_TOKEN so the dev
  // proxy authenticates. Unset = no header.
  configure: (proxy) => {
    const t = process.env.KAERU_VIZ_TOKEN
    if (t) proxy.on('proxyReq', (preq) => preq.setHeader('authorization', `Bearer ${t}`))
  },
})

export default defineConfig({
  base: '/ui/',
  publicDir: false,
  server: {
    proxy: {
      '/v1': daemon(),
      '/auth': daemon(),
      '/admin': daemon(),
    },
  },
  // keep a single three instance shared by the app and three/addons
  // (OrbitControls etc.) — avoids the "multiple instances of three" warning
  resolve: { dedupe: ['three'] },
  build: { assetsInlineLimit: 0, outDir: 'dist', emptyOutDir: true, chunkSizeWarningLimit: 4000, target: 'esnext' },
  esbuild: { target: 'esnext' },
})
