import { fileURLToPath } from 'node:url'

import { defineConfig } from 'vite'

const uiRoot = fileURLToPath(new URL('./code/ui/', import.meta.url))
const fragmentPaths = [
  'src/templates/fragments/dashboard/overview.html',
  'src/templates/fragments/dashboard/overview-today.html',
  'src/templates/fragments/dashboard/overview-lifetime.html',
  'src/templates/fragments/analytics/analytics.html',
  'src/templates/fragments/tops/tops.html',
  'src/templates/fragments/tops/tracks.html',
  'src/templates/fragments/tops/artists.html',
  'src/templates/fragments/tops/albums.html',
  'src/templates/fragments/tops/playlists.html',
  'src/templates/fragments/tops/genres.html',
  'src/templates/fragments/history/history.html',
  'src/templates/fragments/history/history-list.html',
  'src/templates/fragments/history/history-evening.html',
  'src/templates/fragments/quality/quality.html',
  'src/templates/fragments/quality/recheck.html',
  'src/templates/fragments/search/results.html',
  'src/templates/fragments/entities/artist.html',
  'src/templates/fragments/entities/track.html',
  'src/templates/fragments/entities/album.html',
]

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
  },
  build: {
    rollupOptions: {
      input: ['index.html', ...fragmentPaths].map((path) => `${uiRoot}${path}`),
    },
  },
})
