import { defineConfig } from 'vite'

// In dev, proxy /graph.json to the live kaeru-mcp daemon so the galaxy is
// always fresh. For the talk, `npm run build` bundles public/graph.json (the
// baked snapshot) so the app works with no daemon at all.
export default defineConfig({
  server: {
    proxy: {
      '/graph.json': {
        target: process.env.KAERU_VIZ_URL || 'http://127.0.0.1:9876',
        changeOrigin: true,
        // When the daemon requires a bearer token, set KAERU_VIZ_TOKEN so the
        // dev proxy authenticates the live /graph.json fetch. Unset = no header
        // (the app then falls back to the baked public/graph.json snapshot).
        configure: (proxy) => {
          const t = process.env.KAERU_VIZ_TOKEN
          if (t) proxy.on('proxyReq', (preq) => preq.setHeader('authorization', `Bearer ${t}`))
        },
      },
    },
  },
  // keep a single three instance shared by the app and three/addons
  // (OrbitControls etc.) — avoids the "multiple instances of three" warning
  resolve: { dedupe: ['three'] },
  build: { outDir: 'dist', chunkSizeWarningLimit: 4000, target: 'esnext' },
  esbuild: { target: 'esnext' },
})
