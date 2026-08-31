# NovaSound

NovaSound runs PostgreSQL, Axum, and the Bun/Vite/HTMX frontend in Docker. Tauri runs on the Linux host so it can open the native desktop window.

## Linux setup

`install.sh` currently supports CachyOS and Arch Linux. It installs Docker, Make, Rust, Tauri's system dependencies, and the Tauri CLI. It deliberately does not install Bun, Node, npm, or Vite on the host.

```sh
./install.sh
```

If the script adds your account to the `docker` group, log out and back in before continuing.

## Development

```sh
make up
make up-frontend
cd frontend
cargo tauri dev
```

Open `http://localhost:5173` to use the browser frontend. The Tauri window loads that same Vite server during development.

## Production desktop build

```sh
make build-tauri
```

The build disables `linuxdeploy` stripping because its bundled `strip` does not support the ELF `.relr.dyn` sections used by current Arch Linux libraries.
