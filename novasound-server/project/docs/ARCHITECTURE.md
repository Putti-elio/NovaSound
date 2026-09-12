# Server Architecture

`novasound-server/` is the private server workspace. It owns HTTP and ConnectRPC
endpoints, the PostgreSQL schema, provider credentials, and server-only
configuration. Browser and desktop clients consume it as an external service.

## Why This Stack

- **Rust** provides a compiled, type-safe server with explicit ownership of
  resources and errors.
- **Axum** receives HTTP requests, selects a handler, and builds HTTP responses.
  It is the server's HTTP boundary, not the place for business rules or SQL.
- **ConnectRPC and Protocol Buffers** define typed RPC contracts shared by clients
  and the server. The `.proto` files are the source of truth; generated Rust code
  is never edited by hand.
- **PostgreSQL** stores durable relational data such as artists, albums, and songs.
- **deadpool-postgres** manages reusable database connections instead of creating a
  new connection for every request.
- **Clorinde** generates Rust query types from SQL. This keeps SQL explicit while
  giving Rust code checked query inputs and outputs.
- **Askama and HTMX** support the server-rendered catalogue at `/web/`. Askama
  renders the complete page and HTML fragments; an HTMX client can request and swap
  a fragment such as the artist list without a full page reload.
- **Docker Compose** starts the server and PostgreSQL together with the same local
  configuration used by the project commands.

## Request Flow

There are two public server boundaries:

```text
Browser / desktop client
        |
        +-- HTTP or ConnectRPC --> code/src/ adapter --> application use case
        |                                                |
        |                                                v
        |                                      domain rules / storage port
        |
        +-- GET /web/ --> Askama page or HTMX fragment --> HTML response
                                                         |
                                                         v
                                                    PostgreSQL
```

The Axum binary creates one PostgreSQL pool, registers the ConnectRPC router as the
fallback service, nests the server-rendered router at `/web`, and binds to port
`4000`. The current web router owns only `/`, `/artists`, and `/status` below that
prefix. ConnectRPC adapters call application services; the contracts in
`code/contracts/proto/` define the typed artist, album, and song API.

## Directory Map

```text
novasound-server/
├── Cargo.toml                       # Workspace and executable definitions
├── build.rs                          # Generates ConnectRPC Rust code during Cargo builds
├── code/
│   ├── src/                          # Axum/ConnectRPC/Askama boundary and binary entry points
│   ├── templates/                    # Askama page and HTMX fragment templates
│   ├── crates/
│   │   ├── domain/                   # Business concepts and rules
│   │   ├── application/              # Use cases and ports required by use cases
│   │   ├── storage-postgres/         # PostgreSQL implementations of storage ports
│   │   └── providers/                # External music-provider integrations
│   ├── contracts/proto/              # Hand-maintained ConnectRPC contracts
│   ├── database/                     # Migrations, SQL queries, schema reference, and seed data
│   └── generated/clorinde/           # Generated Rust query crate; do not edit manually
├── project/
│   ├── docker/                       # Compose files, Dockerfiles, image versions, local .env
│   ├── ci/                           # Versions used by CI
│   ├── docs/                         # Developer documentation
│   └── scripts/                      # Developer automation
└── .github/workflows/                # GitHub CI workflows
```

## Where A New File Goes

Use the responsibility of the file, not its name, to choose a directory.

| If you are adding... | Put it in... | Example |
| --- | --- | --- |
| An Axum route or request handler | `code/src/` | `code/src/web/artists.rs` for `GET /web/artists` |
| A complete Askama page | `code/templates/` | `code/templates/home.html` |
| An HTML response loaded by HTMX | `code/templates/fragments/` | `code/templates/fragments/artists.html` |
| A business value or rule with no I/O | `code/crates/domain/src/` | `code/crates/domain/src/artist.rs` |
| A use case that coordinates rules and a port | `code/crates/application/src/` | `code/crates/application/src/create_artist.rs` |
| PostgreSQL query execution or database mapping | `code/crates/storage-postgres/src/` | `code/crates/storage-postgres/src/artist_repository.rs` |
| A Spotify, Deezer, Apple Music, or SoundCloud client | `code/crates/providers/src/` | `code/crates/providers/src/spotify.rs` |
| A public RPC request, response, or service | `code/contracts/proto/` | `code/contracts/proto/artist.proto` |
| A database change | `code/database/migrations/` | `code/database/migrations/0002_add_artist_bio.sql` |
| A query consumed by Clorinde | `code/database/queries/` | `code/database/queries/artists.sql` |
| Local development container configuration | `project/docker/` | `project/docker/docker-compose.dev.yml` |

Do not put a `.proto` file in `generated/`, SQL in an Axum handler, or a provider
access token in `code/`. Generated code belongs only in its generator output and
secrets belong only in ignored environment files.

## Generated And Local Files

`code/generated/clorinde/` is regenerated from the SQL query source. Run
`make generate-clorinde` after changing a query. ConnectRPC output is generated by
`build.rs` into Cargo's `OUT_DIR` when Cargo builds the project; do not copy it into
the repository.

Create `project/docker/.env` from `project/docker/.env.example` for local server
configuration. It may contain credentials and must not be committed.
