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
      },
    },
  },
  build: { outDir: 'dist', chunkSizeWarningLimit: 4000, target: 'esnext' },
  esbuild: { target: 'esnext' },
})
