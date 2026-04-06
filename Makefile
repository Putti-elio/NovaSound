DOCKER_COMPOSE_DEV=.tools/docker_compose_dev.yml

fix-rust:
	docker compose -f $(DOCKER_COMPOSE_DEV) exec rust cargo fix -p rust

## RUN

run-rust-release:
	docker compose -f $(DOCKER_COMPOSE_DEV) exec rust cargo run --release

run-rust:
	docker compose -f $(DOCKER_COMPOSE_DEV) exec rust cargo run 

run-rust-error:
	docker compose -f $(DOCKER_COMPOSE_DEV) exec rust RUST_LOG=error cargo run

## CHECK

check-rust:
	docker compose -f $(DOCKER_COMPOSE_DEV) exec rust cargo check

## LINT

lint-rust:
	docker compose -f $(DOCKER_COMPOSE_DEV) exec rust cargo clippy -- -D warnings

fmt-rust:
	docker compose -f $(DOCKER_COMPOSE_DEV) exec rust cargo fmt

fmt-check-rust:
	docker compose -f $(DOCKER_COMPOSE_DEV) exec rust cargo fmt --check

## UP

up:
	make up-rust && make up-vuejs

up-vuejs:
	docker compose -f $(DOCKER_COMPOSE_DEV) watch vuejs
	
up-rust:
	docker compose -f $(DOCKER_COMPOSE_DEV) up rust -d --remove-orphans 
	-docker compose -f $(DOCKER_COMPOSE_DEV) exec rust cargo build --release

## DOWN

down:
	docker compose -f $(DOCKER_COMPOSE_DEV) down

down-rust:
	docker compose -f $(DOCKER_COMPOSE_DEV) down rust

down-vuejs:
	docker compose -f $(DOCKER_COMPOSE_DEV) down vuejs

## SH

sh-rust:
	docker compose -f $(DOCKER_COMPOSE_DEV) exec -it rust sh

sh-vuejs:
	docker compose -f $(DOCKER_COMPOSE_DEV) exec -it vuejs sh

## DELETE

delete-rust:
	make down-rust
	docker compose -f $(DOCKER_COMPOSE_DEV) rm -f rust
	docker image rm -f tools-rust

delete-vuejs:
	make down-vuejs
	docker compose -f $(DOCKER_COMPOSE_DEV) rm -f vuejs
	docker image rm -f tools-vuejs

delete-all:
	make delete-rust 
	make delete-vuejs

## TEST

test: 
	docker compose -f .tools/docker_compose_dev.yml exec rust cargo test

## PROD

prod:
	docker build -f .docker/backend/Dockerfile.prod -t novasound-rust-prod rust
	docker run -d -p 3000:3000 --name novasound-prod novasound-rust-prod
	
## Import SQL

import-sql:
	@read -p "Chemin du fichier SQL à importer: (rust/data/example/example_database.sql) " sqlfile; \
	docker compose -f $(DOCKER_COMPOSE_DEV) cp $$sqlfile rust:/tmp/import.sql && \
	docker compose -f $(DOCKER_COMPOSE_DEV) exec rust sh -c "sqlite3 data/database.db < /tmp/import.sql && echo 'Import réussi!'" && \
	docker compose -f $(DOCKER_COMPOSE_DEV) exec rust rm /tmp/import.sql
	
## VERSIONS

sync-versions:
	sh .tools/sync-versions.sh

sync-versions-dry:
	sh .tools/sync-versions.sh --dry-run

## HOOKS

install-hooks:
	cp .tools/pre-commit.sh .git/hooks/pre-commit
	chmod +x .git/hooks/pre-commit
	@echo "Pre-commit hook installed."

## CLEAN

clean-git:
	git branch -vv | grep ': gone]' | awk '{print $1}' | xargs git branch -D
