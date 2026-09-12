# NovaSound app

NovaSound's browser UI and Tauri desktop shell live here. It consumes the server
as an external HTTP boundary; server credentials and PostgreSQL configuration do
not belong in this repository.

```sh
cp project/setup/.env.example project/setup/.env
bun install
bun run dev
```

See [project documentation](project/docs/README.md) for commands and
[architecture](project/docs/ARCHITECTURE.md) for the directory layout. Set
`VITE_API_BASE_URL` in `project/setup/.env` to the server origin (for example,
`http://localhost:4000`) before loading live catalogue data.
