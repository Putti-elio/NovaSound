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
- **JavaScript ES modules** keep the current UI lightweight. `src/main.js` is the
  browser entry point and imports the code and CSS needed by the page.
- **HTMX** lets HTML elements request server-rendered fragments and replace part of
  a page. For example, the artist catalogue can request HTML from the server
  without implementing a client-side router or a large SPA state layer.
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
code/ui/index.html --> src/main.js --> HTMX request
                                           |
                                           v
                              NovaSound server HTTP / ConnectRPC boundary

code/desktop/ wraps code/ui/ for desktop-specific capabilities.
code/demo/ reserves a future static demonstration that must use fictional local
data only.
```

The UI can know the server's public URL through `VITE_API_BASE_URL`. It must not
know a database URL, a PostgreSQL password, or a music-provider secret.

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
│   │   │   ├── main.js                # Browser entry point
│   │   │   ├── assets/                # Imported UI assets
│   │   │   ├── styles/                # CSS imported by browser modules
│   │   │   ├── templates/             # HTML fragments grouped by UI feature
│   │   │   └── static/                # Existing standalone static UI assets
│   │   └── dist/                      # Generated Vite output; do not edit
│   ├── desktop/                       # Tauri Rust crate, capabilities, icons, packaging
│   └── demo/                          # Future mock-only demo configuration
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
| Browser startup or global event handling | `code/ui/src/` | `code/ui/src/main.js` |
| A CSS file imported by a UI module | `code/ui/src/styles/` | `code/ui/src/styles/catalogue.css` |
| An image, font, or SVG imported by JavaScript or CSS | `code/ui/src/assets/` | `code/ui/src/assets/logo.svg` |
| A favicon or file that must keep its exact URL | `code/ui/public/` | `code/ui/public/favicon.svg` |
| An HTML fragment owned by a screen or capability | `code/ui/src/templates/` | `code/ui/src/templates/catalogue/artist-card.html` |
| Fictional fixture data for a future static demo | `code/ui/src/mocks/` | `code/ui/src/mocks/artists.js` |
| A Tauri command or desktop permission | `code/desktop/src/` or `code/desktop/capabilities/` | `code/desktop/src/lib.rs` or `code/desktop/capabilities/default.json` |
| Vite build output | `code/ui/dist/` | Generated only; never edit manually |

The UI currently consists of static HTML, JavaScript, CSS, and HTMX requests; it
does not include a client-side router or a framework state store. When adding new UI
code, prefer the directory whose role is explicit in the table above.

## Environment Files

Vite reads browser environment variables from `project/setup/`, configured by
`vite.config.ts`. Copy `project/setup/.env.example` to `project/setup/.env` and use
only `VITE_`-prefixed variables for values intentionally exposed to the browser.
`VITE_API_BASE_URL` is interpolated into the current artist-catalogue request;
configure it with the server origin, not a database connection string or secret.
