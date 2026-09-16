# App Architecture

`novasound-app/` owns the browser user interface and its Tauri desktop wrapper. It
does not own server credentials, PostgreSQL access, or provider tokens: those stay
in `novasound-server/`.

## Why This Stack

- **Bun** installs dependencies and runs the frontend scripts quickly. `bun.lock`
  records the exact dependency versions used by the project.
- **Vite** runs the local development server and produces the static browser build.
  Its root is `code/ui/`, so `index.html` and browser source live there rather than
  at the repository root.
- **JavaScript ES modules** keep the current UI lightweight.
  `src/static/js/app.js` is the browser entry point and imports HTMX and the shared
  stylesheet.
- **HTMX** lets HTML elements request and replace small HTML fragments without a
  client-side router or a large SPA state layer. The current prototype serves
  fictional fragments from the Vite build and does not call the server.
- **Tauri** packages the same browser UI as a desktop application. Its Rust crate
  stays thin: add Rust only for a real native capability such as a filesystem,
  window, notification, or OS integration need.
- **Docker** is optional for this repository. It packages Bun/Vite only and never
  bundles the server database or server credentials.

## Runtime Boundaries

```text
Browser build or Tauri webview
        |
        v
code/ui/index.html --> src/static/js/app.js --> local HTMX fragments

code/desktop/ wraps code/ui/ for desktop-specific capabilities.
code/ui/src/mocks/ documents the fictional data represented by the demo.
```

Future live integrations may expose the server's public URL through a `VITE_`
variable. The UI must not know a database URL, a PostgreSQL password, or a
music-provider secret.

## Directory Map

```text
novasound-app/
├── package.json                       # Bun scripts and browser dependencies
├── bun.lock                           # Locked dependency versions
├── vite.config.ts                     # Vite root, environment directory, dev-server settings
├── code/
│   ├── ui/                            # Shared browser UI and Vite root
│   │   ├── index.html                 # Browser document and module entry reference
│   │   ├── public/                    # Files copied unchanged to the build output
│   │   ├── src/
│   │   │   ├── assets/                # Imported UI assets
│   │   │   ├── templates/             # HTML fragments grouped by UI feature
│   │   │   ├── mocks/                 # Fictional datasets represented by the demo
│   │   │   └── static/                # Browser entry point and shared styles
│   │   └── dist/                      # Generated Vite output; do not edit
│   ├── desktop/                       # Tauri Rust crate, capabilities, icons, packaging
├── project/
│   ├── docker/                        # Bun/Vite container configuration
│   ├── setup/                         # Local browser environment file template
│   ├── ci/                            # Versions used by CI
│   └── docs/                          # Developer documentation
└── .github/workflows/                 # GitHub CI workflows
```

## Where A New File Goes

| If you are adding... | Put it in... | Example |
| --- | --- | --- |
| Browser startup or global event handling | `code/ui/src/static/js/` | `code/ui/src/static/js/app.js` |
| Shared UI styles | `code/ui/src/static/css/` | `code/ui/src/static/css/app.css` |
| An image, font, or SVG imported by JavaScript or CSS | `code/ui/src/assets/` | `code/ui/src/assets/logo.svg` |
| A favicon or file that must keep its exact URL | `code/ui/public/` | `code/ui/public/favicon.svg` |
| An HTML fragment owned by a screen or capability | `code/ui/src/templates/` | `code/ui/src/templates/catalogue/artist-card.html` |
| Fictional fixture data represented by the static demo | `code/ui/src/mocks/` | `code/ui/src/mocks/artists.json` |
| A Tauri command or desktop permission | `code/desktop/src/` or `code/desktop/capabilities/` | `code/desktop/src/lib.rs` or `code/desktop/capabilities/default.json` |
| Vite build output | `code/ui/dist/` | Generated only; never edit manually |

The UI currently consists of static HTML, JavaScript, CSS, and HTMX requests; it
does not include a client-side router or a framework state store. When adding new UI
code, prefer the directory whose role is explicit in the table above.

## Environment Files

Vite reads browser environment variables from `project/setup/`, configured by
`vite.config.ts`. Copy `project/setup/.env.example` to `project/setup/.env` and use
only `VITE_`-prefixed variables for values intentionally exposed to the browser.
The current static demo does not require an environment variable.
