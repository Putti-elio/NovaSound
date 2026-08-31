DOCKER_DIR := setup/docker
ENV_FILE := $(DOCKER_DIR)/.env
VERSIONS_FILE := $(DOCKER_DIR)/images/versions
MAKEFILE_DOC := docs/MAKEFILE.md

DOCKER_POSTGRES_HOST ?= postgres_db
FRONTEND_URL ?= http://localhost:5173
API_BASE_URL ?= http://127.0.0.1:4000

ifneq ($(wildcard $(ENV_FILE)),)
include $(ENV_FILE)
export
endif

APP_ENV ?= dev

COMPOSE_FILES := -f $(DOCKER_DIR)/compose.yml
ifeq ($(APP_ENV),dev)
COMPOSE_FILES += -f $(DOCKER_DIR)/docker-compose.dev.yml
else ifeq ($(APP_ENV),prod)
COMPOSE_FILES += -f $(DOCKER_DIR)/docker-compose.prod.yml
endif

COMPOSE = docker compose \
	$(COMPOSE_FILES) \
	--env-file $(VERSIONS_FILE) \
	--env-file $(ENV_FILE) \
	--project-directory .

PROD_COMPOSE = docker compose \
	-f $(DOCKER_DIR)/compose.yml \
	-f $(DOCKER_DIR)/docker-compose.prod.yml \
	--env-file $(VERSIONS_FILE) \
	--env-file $(ENV_FILE) \
	--project-directory .

DATABASE_URL_DOCKER = postgres://$${POSTGRES_USER}:$${POSTGRES_PASSWORD}@$(DOCKER_POSTGRES_HOST):$${POSTGRES_PORT}/$${POSTGRES_DB}
TEST_DATABASE_URL_DOCKER = postgres://$${POSTGRES_USER}:$${POSTGRES_PASSWORD}@$(DOCKER_POSTGRES_HOST):$${POSTGRES_PORT}/$${POSTGRES_DB}_test
ENSURE_DATABASE = $(COMPOSE) exec -T postgres_db sh -c 'createdb -U "$${POSTGRES_USER}" "$${POSTGRES_DB}" 2>/dev/null || psql -U "$${POSTGRES_USER}" -d "$${POSTGRES_DB}" -c "SELECT 1" >/dev/null'
MIGRATE_DATABASE = $(COMPOSE) exec -T -e POSTGRES_USER -e POSTGRES_PASSWORD -e POSTGRES_DB -e POSTGRES_PORT backend sh -c 'DATABASE_URL="$(DATABASE_URL_DOCKER)" cargo run --bin database_setup'
RESET_DATABASE = $(COMPOSE) exec -T -e POSTGRES_USER -e POSTGRES_PASSWORD -e POSTGRES_DB -e POSTGRES_PORT backend sh -c 'DATABASE_URL="$(DATABASE_URL_DOCKER)" cargo run --bin database_setup -- --reset'
SEED_DATABASE = $(COMPOSE) exec -T postgres_db psql -U $(POSTGRES_USER) -d $(POSTGRES_DB) < backend/database/seed.sql

.PHONY: \
	up up-frontend up-backend up-db \
	down down-frontend down-backend down-db \
	restart restart-frontend restart-backend restart-db \
	run-backend run-backend-release run-backend-error \
	init-db seed-db reset-db generate-clorinde export-db import-db \
	check-backend lint lint-backend fmt-backend fmt-check-backend fix-backend test \
	build-frontend build-tauri build-prod prod \
	sh-frontend sh-backend sh-db \
	clear clear-frontend clear-backend clear-db clean-git \
	tag docs

## @category Development

## @description Start all development containers
## @depends up-backend, up-frontend
up: up-backend up-frontend

## @description Start the frontend container with Vite hot reload
up-frontend:
	$(COMPOSE) up frontend -d --build --remove-orphans
	@printf "\nNovaSound frontend: $(FRONTEND_URL)\n"
	@printf "Backend status:     $(API_BASE_URL)/web/status\n"
	@printf "Artist fragment:    $(API_BASE_URL)/web/artists\n\n"

## @description Start the backend and PostgreSQL containers
up-backend:
	$(COMPOSE) up backend postgres_db -d --build --remove-orphans

## @description Start only the PostgreSQL container
up-db:
	$(COMPOSE) up postgres_db -d --build --remove-orphans

## @description Run the backend in development mode
## @depends up-backend
run-backend: up-backend
	$(COMPOSE) exec backend cargo run --bin rust

## @description Run the backend in release mode
## @depends up-backend
run-backend-release: up-backend
	$(COMPOSE) exec backend cargo run --release --bin rust

## @description Run the backend with error-only logging
## @depends up-backend
run-backend-error: up-backend
	$(COMPOSE) exec backend env RUST_LOG=error cargo run --bin rust

## @description Stop all containers
down:
	$(COMPOSE) down

## @description Stop the frontend container
down-frontend:
	$(COMPOSE) stop frontend

## @description Stop the backend container
down-backend:
	$(COMPOSE) stop backend

## @description Stop the PostgreSQL container
down-db:
	$(COMPOSE) stop postgres_db

## @description Restart all development containers
## @depends down, up
restart: down up

## @description Restart the frontend container
## @depends down-frontend, up-frontend
restart-frontend: down-frontend up-frontend

## @description Restart the backend and PostgreSQL containers
## @depends down-backend, up-backend
restart-backend: down-backend up-backend

## @description Restart the PostgreSQL container
## @depends down-db, up-db
restart-db: down-db up-db

## @category Database

## @description Apply database migrations and load demo data
## @depends up-backend
init-db: up-backend
	$(ENSURE_DATABASE)
	$(MIGRATE_DATABASE)
	$(SEED_DATABASE)

## @description Load demo artists, albums, and songs
## @depends up-backend
seed-db: up-backend
	$(ENSURE_DATABASE)
	$(MIGRATE_DATABASE)
	$(SEED_DATABASE)

## @description Recreate the database schema and reload demo data
## @depends up-backend
reset-db: up-backend
	$(ENSURE_DATABASE)
	$(RESET_DATABASE)
	$(SEED_DATABASE)

## @description Regenerate Clorinde Rust query types from SQL files
## @depends init-db
generate-clorinde:
	$(COMPOSE) exec -T backend sh -c 'command -v clorinde >/dev/null 2>&1 || cargo install clorinde'
	$(ENSURE_DATABASE)
	$(COMPOSE) exec -T -e POSTGRES_USER -e POSTGRES_PASSWORD -e POSTGRES_DB -e POSTGRES_PORT -u $(shell id -u):$(shell id -g) backend sh -c 'clorinde live "$(DATABASE_URL_DOCKER)"'
	$(COMPOSE) exec -T backend chown -R $(shell id -u):$(shell id -g) clorinde
	$(COMPOSE) exec -T -u $(shell id -u):$(shell id -g) backend cargo fmt --manifest-path clorinde/Cargo.toml

## @description Export PostgreSQL data to a SQL file
export-db:
	@export_file="$(EXPORT_FILE)"; \
	if [ -z "$$export_file" ]; then read -r -p "Export path (default: backend/data/export_$$(date +%Y%m%d_%H%M%S).sql): " export_file; fi; \
	export_file=$${export_file:-backend/data/export_$$(date +%Y%m%d_%H%M%S).sql}; \
	$(COMPOSE) exec -T postgres_db pg_dump -U $(POSTGRES_USER) -d $(POSTGRES_DB) > "$$export_file"; \
	printf "Database exported to %s\n" "$$export_file"

## @description Import a SQL file into PostgreSQL
import-db:
	@import_file="$(IMPORT_FILE)"; \
	if [ -z "$$import_file" ]; then read -r -p "Path of SQL file to import: " import_file; fi; \
	test -n "$$import_file" || { printf "An import file is required.\n"; exit 1; }; \
	$(COMPOSE) cp "$$import_file" postgres_db:/tmp/import.sql; \
	$(COMPOSE) exec -T postgres_db psql -U $(POSTGRES_USER) -d $(POSTGRES_DB) -f /tmp/import.sql; \
	$(COMPOSE) exec -T postgres_db rm -f /tmp/import.sql; \
	printf "Database imported from %s\n" "$$import_file"

## @category Quality

## @description Check that the backend compiles
## @depends up-backend
check-backend: up-backend
	$(COMPOSE) exec backend cargo check

## @description Run backend formatting and Clippy checks used by CI
## @depends up-backend
lint: up-backend
	$(COMPOSE) exec -T backend cargo fmt --all -- --check
	$(COMPOSE) exec -T backend cargo clippy --all-targets -- -D warnings

## @description Run Clippy on the backend
## @depends up-backend
lint-backend: up-backend
	$(COMPOSE) exec -T backend cargo clippy -- -D warnings

## @description Format backend Rust code
## @depends up-backend
fmt-backend: up-backend
	$(COMPOSE) exec -T backend cargo fmt

## @description Check backend Rust formatting
## @depends up-backend
fmt-check-backend: up-backend
	$(COMPOSE) exec -T backend cargo fmt --check

## @description Apply automatic Rust fixes to the backend
## @depends up-backend
fix-backend: up-backend
	$(COMPOSE) exec backend cargo fix -p rust

## @description Run all backend tests
## @depends up-backend
test: up-backend
	$(COMPOSE) exec -T postgres_db sh -c 'test_db="$${POSTGRES_DB}_test"; createdb -U "$${POSTGRES_USER}" "$$test_db" 2>/dev/null || psql -U "$${POSTGRES_USER}" -d "$$test_db" -c "SELECT 1" >/dev/null'
	$(COMPOSE) exec -T -e POSTGRES_USER -e POSTGRES_PASSWORD -e POSTGRES_DB -e POSTGRES_PORT backend sh -c 'TEST_DATABASE_URL="$(TEST_DATABASE_URL_DOCKER)" cargo test --workspace --all-targets -- --test-threads=1'
	$(COMPOSE) exec -T -e POSTGRES_USER -e POSTGRES_PASSWORD -e POSTGRES_DB -e POSTGRES_PORT backend sh -c 'TEST_DATABASE_URL="$(TEST_DATABASE_URL_DOCKER)" cargo test --workspace --doc'

## @category Build

## @description Build frontend assets in Docker for Tauri
build-frontend:
	$(COMPOSE) run --rm frontend sh -c 'bun install --frozen-lockfile && bun run build'

## @description Build Linux desktop packages on the host
## @depends build-frontend
build-tauri: build-frontend
	cd frontend && NO_STRIP=1 cargo tauri build

## @description Build the production backend image
build-prod:
	@tag=$$(git describe --tags --abbrev=0 2>/dev/null || printf latest); \
	printf "Building backend image with tag: %s\n" "$$tag"; \
	APP_ENV=prod TAG="$$tag" $(PROD_COMPOSE) build backend

## @description Build and start the production backend and PostgreSQL services
## @depends build-prod
prod: build-prod
	APP_ENV=prod $(PROD_COMPOSE) up -d backend postgres_db

## @category Shell

## @description Open a shell in the frontend container
sh-frontend:
	$(COMPOSE) exec frontend sh

## @description Open a shell in the backend container
sh-backend:
	$(COMPOSE) exec backend sh

## @description Open a PostgreSQL shell
sh-db:
	$(COMPOSE) exec postgres_db psql -U $(POSTGRES_USER) -d $(POSTGRES_DB)

## @category Cleanup

## @description Delete all application containers, images, and data volumes
## @depends clear-frontend, clear-backend, clear-db
clear: clear-frontend clear-backend clear-db

## @description Delete the frontend container, image, and dependency volume
clear-frontend:
	$(COMPOSE) rm -sf frontend
	-docker volume rm novasound_frontend_node_modules
	-docker image rm -f novasound/frontend-dev:latest

## @description Delete backend containers and images
clear-backend:
	$(COMPOSE) rm -sf backend
	-docker image rm -f novasound/backend-dev:latest
	-docker image rm -f novasound/backend-prod:latest

## @description Delete PostgreSQL containers, images, and data volumes
clear-db:
	$(COMPOSE) rm -sf postgres_db
	-docker volume rm novasound_postgres_data
	-docker image rm -f novasound/postgres-dev:latest
	-docker image rm -f novasound/postgres-prod:latest

## @description Delete local branches whose remote branch was removed
clean-git:
	@git branch -vv | grep ': gone]' | awk '{print $$1}' | xargs -r git branch -D

## @category Release

## @description Create an annotated version tag, for example TAG=v1.2.3
## @env TAG
tag:
	@test -n "$(TAG)" || { printf "Usage: make tag TAG=v1.2.3\n"; exit 1; }
	git tag -a "$(TAG)" -m "Release $(TAG)"
	@printf "Tag %s created. Push with: git push origin %s\n" "$(TAG)" "$(TAG)"

## @category Documentation

## @description Generate Markdown documentation from the Makefile
docs:
	@command -v makefile2doc >/dev/null 2>&1 || { \
		printf "makefile2doc is required. Install it with:\n"; \
		printf "cargo install --git https://github.com/Merlin-Clos/makefile2doc --locked\n"; \
		exit 1; \
	}
	makefile2doc --input Makefile --output $(MAKEFILE_DOC)
