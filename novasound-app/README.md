# NovaSound app

NovaSound's browser UI and Tauri desktop shell live here. The current HTMX
prototype loads local HTML fragments backed by local mock listening data. It
contains 100 artists from Billboard's [2025 Top Artists](https://www.billboard.com/charts/year-end/2025/top-artists/),
plus 100 real tracks, albums, and public playlists from Deezer's public chart
responses. It does not connect to the server or a streaming service. Server
credentials and PostgreSQL configuration do not belong in this repository.

```sh
cp project/setup/.env.example project/setup/.env
bun install
bun run dev
```

See [project documentation](project/docs/README.md) for commands and
[architecture](project/docs/ARCHITECTURE.md) for the directory layout. The demo
is available at `http://localhost:5173` and identifies itself with the `Mode demo`
badge.
