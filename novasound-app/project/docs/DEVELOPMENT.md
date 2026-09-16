# App Development Guide

Read [Architecture](ARCHITECTURE.md) for the reason behind the directory layout.
This guide explains the normal frontend workflow and where a junior developer
should place a change.

## Start The Browser UI

From the repository root:

```sh
cp project/setup/.env.example project/setup/.env
bun install
bun run dev
```

Vite listens on port `5173`. The current prototype requests only local HTML
fragments under `code/ui/src/templates/fragments/`; it does not require the server.
Do not put a password, API secret, or database connection string in
`project/setup/.env`: every `VITE_` value is visible in the browser build.

## How To Place A UI Change

### Change What The Page Does

Start from `code/ui/src/static/js/app.js` when the change concerns browser startup
or a global HTMX event. This entry point loads HTMX and the shared stylesheet.

Put page-specific markup in `code/ui/src/templates/`, grouped by feature. Shared
styles live in `code/ui/src/static/css/`.

### Add Assets And Demo Data

Put an imported logo, illustration, or font in `code/ui/src/assets/`. Put a file
that must be requested by a fixed URL, such as `favicon.svg`, in `code/ui/public/`.

The mock fixtures in `code/ui/src/mocks/` document the fictional data represented
by the static demo. They must never require login or contain a real user's
listening data.

### Add Desktop Support

Keep ordinary HTML, CSS, and JavaScript in `code/ui/` so the browser and desktop
application share it. Add a file in `code/desktop/` only when the feature requires
a native capability. Examples include a Tauri command in `code/desktop/src/`, a
new permission in `code/desktop/capabilities/`, or a desktop icon in
`code/desktop/icons/`.

## Validate A Change

Run from the repository root:

```sh
bun run build
(cd code/desktop && cargo check)
```

Run `bun run build` for every browser change. Run the Tauri check as well when you
change shared UI assets used by desktop or anything under `code/desktop/`.

The package scripts are `dev`, `build`, and `preview`; there is currently no test
script or browser test suite. `make dev` and `make build` install with the lockfile
and run the corresponding Bun scripts. `make dev-docker` runs the Vite service in
Docker, and `make build-tauri` first builds browser assets then runs `cargo tauri
build` from `code/desktop/`.
