# NovaSound frontend

This is a framework-free HTML frontend using HTMX and Vite. Bun 1.4 and Vite run only in Docker; do not install Bun, Node, npm, or Vite on the host.

```sh
make up-frontend
```

The development server is available at `http://localhost:5173`. Build production assets from Docker with `make build-frontend`; the output is written to `frontend/dist/` for the host-side Tauri build.
