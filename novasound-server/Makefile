DOCKER_DIR := project/docker
ENV_FILE := $(DOCKER_DIR)/.env
VERSIONS_FILE := $(DOCKER_DIR)/images/versions
MAKEFILE_DOC := project/docs/MAKEFILE.md

DOCKER_POSTGRES_HOST ?= postgres_db

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

COMPOSE = docker compose $(COMPOSE_FILES) --env-file $(VERSIONS_FILE) --env-file $(ENV_FILE) --project-directory .
PROD_COMPOSE = docker compose -f $(DOCKER_DIR)/compose.yml -f $(DOCKER_DIR)/docker-compose.prod.yml --env-file $(VERSIONS_FILE) --env-file $(ENV_FILE) --project-directory .
DATABASE_URL_DOCKER = postgres://$${POSTGRES_USER}:$${POSTGRES_PASSWORD}@$(DOCKER_POSTGRES_HOST):$${POSTGRES_PORT}/$${POSTGRES_DB}
TEST_DATABASE_URL_DOCKER = postgres://$${POSTGRES_USER}:$${POSTGRES_PASSWORD}@$(DOCKER_POSTGRES_HOST):$${POSTGRES_PORT}/$${POSTGRES_DB}_test
ENSURE_DATABASE = $(COMPOSE) exec -T postgres_db sh -c 'createdb -U "$${POSTGRES_USER}" "$${POSTGRES_DB}" 2>/dev/null || psql -U "$${POSTGRES_USER}" -d "$${POSTGRES_DB}" -c "SELECT 1" >/dev/null'
MIGRATE_DATABASE = $(COMPOSE) exec -T -e POSTGRES_USER -e POSTGRES_PASSWORD -e POSTGRES_DB -e POSTGRES_PORT backend sh -c 'DATABASE_URL="$(DATABASE_URL_DOCKER)" cargo run --bin database_setup'
RESET_DATABASE = $(COMPOSE) exec -T -e POSTGRES_USER -e POSTGRES_PASSWORD -e POSTGRES_DB -e POSTGRES_PORT backend sh -c 'DATABASE_URL="$(DATABASE_URL_DOCKER)" cargo run --bin database_setup -- --reset'

.PHONY: up down up-backend up-db down-backend down-db restart restart-backend restart-db run-backend run-backend-release run-backend-error init-db seed-db reset-db generate-clorinde generate-protocol generate export-db import-db check-backend lint lint-backend fmt-backend fmt-check-backend fix-backend test build-prod prod sh-backend sh-db clear clear-backend clear-db tag docs

## @category Development

## @description Start the backend and PostgreSQL containers
up: up-backend

up-backend:
	$(COMPOSE) up backend postgres_db -d --build --remove-orphans

## @description Start only PostgreSQL
up-db:
	$(COMPOSE) up postgres_db -d --build --remove-orphans

## @description Run the backend in development mode
## @depends up-backend
run-backend: up-backend
	$(COMPOSE) exec backend cargo run --bin rust

run-backend-release: up-backend
	$(COMPOSE) exec backend cargo run --release --bin rust

run-backend-error: up-backend
	$(COMPOSE) exec backend env RUST_LOG=error cargo run --bin rust

## @description Stop backend and PostgreSQL containers
down:
	$(COMPOSE) down

down-backend:
	$(COMPOSE) stop backend

down-db:
	$(COMPOSE) stop postgres_db

restart: down up
restart-backend: down-backend up-backend
restart-db: down-db up-db

## @category Database

init-db: up-backend
	$(ENSURE_DATABASE)
	$(MIGRATE_DATABASE)
	$(COMPOSE) exec -T postgres_db psql -U $(POSTGRES_USER) -d $(POSTGRES_DB) < code/database/seed.sql

seed-db: init-db

reset-db: up-backend
	$(ENSURE_DATABASE)
	$(RESET_DATABASE)
	$(COMPOSE) exec -T postgres_db psql -U $(POSTGRES_USER) -d $(POSTGRES_DB) < code/database/seed.sql

generate-clorinde: init-db
	$(COMPOSE) exec -T backend sh -c 'command -v clorinde >/dev/null 2>&1 || cargo install clorinde'
	$(COMPOSE) exec -T -e POSTGRES_USER -e POSTGRES_PASSWORD -e POSTGRES_DB -e POSTGRES_PORT -u $(shell id -u):$(shell id -g) backend sh -c 'clorinde live "$(DATABASE_URL_DOCKER)"'
	$(COMPOSE) exec -T backend chown -R $(shell id -u):$(shell id -g) code/generated/clorinde
	$(COMPOSE) exec -T -u $(shell id -u):$(shell id -g) backend cargo fmt --manifest-path code/generated/clorinde/Cargo.toml

generate-protocol:
	$(COMPOSE) exec -T backend cargo build

generate: generate-clorinde generate-protocol

export-db:
	@export_file="$${EXPORT_FILE:-code/database/export_$$(date +%Y%m%d_%H%M%S).sql}"; $(COMPOSE) exec -T postgres_db pg_dump -U $(POSTGRES_USER) -d $(POSTGRES_DB) > "$$export_file"

import-db:
	@test -n "$(IMPORT_FILE)" || { printf "Usage: make import-db IMPORT_FILE=path.sql\n"; exit 1; }
	$(COMPOSE) cp "$(IMPORT_FILE)" postgres_db:/tmp/import.sql
	$(COMPOSE) exec -T postgres_db psql -U $(POSTGRES_USER) -d $(POSTGRES_DB) -f /tmp/import.sql
	$(COMPOSE) exec -T postgres_db rm -f /tmp/import.sql

## @category Quality

check-backend: up-backend
	$(COMPOSE) exec backend cargo check

lint: up-backend
	$(COMPOSE) exec -T backend cargo fmt --all -- --check
	$(COMPOSE) exec -T backend cargo clippy --all-targets -- -D warnings

lint-backend: up-backend
	$(COMPOSE) exec -T backend cargo clippy -- -D warnings

fmt-backend: up-backend
	$(COMPOSE) exec -T backend cargo fmt

fmt-check-backend: up-backend
	$(COMPOSE) exec -T backend cargo fmt --check

fix-backend: up-backend
	$(COMPOSE) exec backend cargo fix -p novasound-server

test: up-backend
	$(COMPOSE) exec -T postgres_db sh -c 'test_db="$${POSTGRES_DB}_test"; createdb -U "$${POSTGRES_USER}" "$$test_db" 2>/dev/null || psql -U "$${POSTGRES_USER}" -d "$$test_db" -c "SELECT 1" >/dev/null'
	$(COMPOSE) exec -T -e POSTGRES_USER -e POSTGRES_PASSWORD -e POSTGRES_DB -e POSTGRES_PORT backend sh -c 'TEST_DATABASE_URL="$(TEST_DATABASE_URL_DOCKER)" cargo test --workspace --all-targets -- --test-threads=1'
	$(COMPOSE) exec -T -e POSTGRES_USER -e POSTGRES_PASSWORD -e POSTGRES_DB -e POSTGRES_PORT backend sh -c 'TEST_DATABASE_URL="$(TEST_DATABASE_URL_DOCKER)" cargo test --workspace --doc'

## @category Build

build-prod:
	@tag=$$(git describe --tags --abbrev=0 2>/dev/null || printf latest); APP_ENV=prod TAG="$$tag" $(PROD_COMPOSE) build backend

prod: build-prod
	APP_ENV=prod $(PROD_COMPOSE) up -d backend postgres_db

## @category Shell

sh-backend:
	$(COMPOSE) exec backend sh

sh-db:
	$(COMPOSE) exec postgres_db psql -U $(POSTGRES_USER) -d $(POSTGRES_DB)

## @category Cleanup

clear: clear-backend clear-db

clear-backend:
	$(COMPOSE) rm -sf backend
	-docker image rm -f novasound/server-dev:latest novasound/server-prod:latest

clear-db:
	$(COMPOSE) rm -sf postgres_db
	-docker volume rm novasound_postgres_data
	-docker image rm -f novasound/postgres-dev:latest novasound/postgres-prod:latest

## @category Release

tag:
	@test -n "$(TAG)" || { printf "Usage: make tag TAG=v1.2.3\n"; exit 1; }
	git tag -a "$(TAG)" -m "Release $(TAG)"

docs:
	@command -v makefile2doc >/dev/null 2>&1 || { printf "makefile2doc is required.\n"; exit 1; }
	makefile2doc --input Makefile --output $(MAKEFILE_DOC)
