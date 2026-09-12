import { defineConfig } from 'vite'

export default defineConfig({
  root: 'code/ui',
  envDir: '../../project/setup',
  server: {
    host: '0.0.0.0',
    port: 5173,
    strictPort: true,
    watch: {
      usePolling: true,
    },
    hmr: {
      host: 'localhost',
    },
  },
})
