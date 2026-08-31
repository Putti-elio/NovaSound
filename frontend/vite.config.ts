import { defineConfig } from 'vite'

export default defineConfig({
  envPrefix: ['VITE_', 'API_'],
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
