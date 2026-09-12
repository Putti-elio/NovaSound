# Server Development Guide

Read [Architecture](ARCHITECTURE.md) before creating a new module. This guide is
the practical companion: it explains how to place and validate day-to-day changes.

## Before You Change Code

1. Start PostgreSQL and the development container with `make up`.
2. Apply the database schema and seed data with `make init-db`.
3. Open the backend container with `make sh-backend`, then run
   `cargo run --bin novasound-server`.
4. Use the `code/` directories that own the responsibility you are changing.

The local environment file is `project/docker/.env`, copied from
`project/docker/.env.example`. Keep database passwords, provider tokens, and other
private values there, never in Rust source, templates, or committed documentation.

## Common Changes

### Current HTTP And ConnectRPC Boundaries

The server binds to port `4000`. Its hand-maintained contracts are
`code/contracts/proto/{artist,album,song}.proto`; Cargo generates their Rust
bindings through `build.rs`. The registered ConnectRPC services expose create, get,
list, update, and delete operations for artists, albums, and songs. Treat the
Protobuf files as the public typed API source of truth rather than generated Rust
types or an inferred HTTP route.

The explicit HTML routes are `GET /web/`, `GET /web/artists`, and `GET /web/status`.
The app currently requests `/web/artists` with HTMX. Development CORS permits
`http://localhost:5173` and `tauri://localhost`; outside development, `WEB_ORIGIN`
is required by the server.

### Add A Server-Rendered Catalogue Fragment

For a new server-rendered fragment, add an Askama template under
`code/templates/fragments/`, a handler under `code/src/web/`, and a route in the
web router. For example, an albums fragment can use:

```text
code/templates/fragments/albums.html
code/src/web/albums.rs
```

The handler loads data through an application use case, renders the template, and
returns HTML. The template may contain presentation markup and HTMX attributes. It
must not open database connections or contain Rust business rules.

### Add An API Or RPC Capability

Start from the contract and use case, then add the transport adapter:

1. Define or update the request and response in `code/contracts/proto/` when the
   capability is exposed through ConnectRPC.
2. Put business decisions in `code/crates/domain/` and orchestration in
   `code/crates/application/`.
3. Implement PostgreSQL access in `code/crates/storage-postgres/`.
4. Connect the HTTP or ConnectRPC handler in `code/src/`.
5. Regenerate protocol output with `make generate-protocol` and query output with
   `make generate-clorinde` when their source files changed.

### Change The Database

Create a new numbered migration in `code/database/migrations/`; do not change an
already-applied migration. Put reusable SQL queries in `code/database/queries/`.
`schema.sql` is a readable schema reference, while migrations are what the database
setup executable applies. Update both deliberately.

The current catalog schema is deliberately small:

| Table | Required data | Relationships |
| --- | --- | --- |
| `artists` | text `id`, unique `name` | An artist can own albums and songs. |
| `albums` | text `id`, `name`, `artist_id` | The artist foreign key cascades on delete. `total_duration`, `release_date`, `image_path`, and `album_type` use the migration's defaults or nullability. |
| `songs` | text `id`, `name`, `artist_id` | `artist_id` references an artist. `album_id` is optional and is set to `NULL` when its album is deleted. |

`songs.album_id` and `songs.artist_id` have indexes. The API represents optional
album and song dates as `DD-MM-YYYY`; invalid optional dates are rejected at the
ConnectRPC boundary.

## Commands

Run these from the repository root:

```sh
make lint              # rustfmt and Clippy in the backend container
make test              # workspace tests against the test PostgreSQL database
make check-backend     # fast Cargo type check
make generate-clorinde # regenerate Clorinde after SQL-query changes
make generate-protocol # run Cargo so build.rs regenerates ConnectRPC output
make generate          # regenerate both kinds of generated code
```

Run `make lint` and `make test` before opening a pull request. If you change a
database migration, also run `make reset-db` locally to prove a fresh database can
be created.

`make test` creates or reuses `${POSTGRES_DB}_test` and runs workspace target and
documentation tests serially. The ConnectRPC integration tests cover CRUD flows for
all three catalog services plus not-found and invalid-date errors.

`make run-backend` currently targets a binary named `rust`, but the manifest names
the server binary `novasound-server`. Until that Makefile target is corrected, use
the container command shown above.
