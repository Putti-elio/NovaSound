# NovaSound server

This repository owns the Axum/ConnectRPC server, PostgreSQL development stack,
database schema, and server-only configuration. The NovaSound app is an external
client; it does not own this server's credentials or database lifecycle.

```sh
cp project/docker/.env.example project/docker/.env
make up
make init-db
make sh-backend
# In the container:
cargo run --bin novasound-server
```

The server listens on `http://localhost:4000`. The browser catalogue currently
loads `GET /web/artists`; the typed artist, album, and song APIs are defined by the
Protobuf contracts in `code/contracts/proto/`. See the
[development guide](project/docs/DEVELOPMENT.md),
[architecture](project/docs/ARCHITECTURE.md), and
[installation guide](project/docs/INSTALLATION.md).

If you already have `project/setup/.env`, it is not moved automatically. Manually
migrate it to `project/docker/.env` before running server commands.

Run `make lint` and `make test` for the local Rust quality and database-backed test
checks. Run `make build-prod` to build the production server image.

`make run-backend` currently targets an outdated binary name. Use the command above
until that Makefile target is corrected.
