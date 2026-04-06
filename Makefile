DOCKER_COMPOSE_DEV=.tools/docker_compose_dev.yml 
DOCKERFILE_DEV_RUST=rust/.Docker/Dockerfile.dev

fix-rust:
	docker compose -f $(DOCKER_COMPOSE_DEV) exec rust  cargo fix -p rust

## RUN

run-rust-release:
	docker compose -f $(DOCKER_COMPOSE_DEV) exec rust cargo run --release

run-rust:
	docker compose -f $(DOCKER_COMPOSE_DEV) exec rust cargo run 

run-rust-error:
	docker compose -f $(DOCKER_COMPOSE_DEV) exec rust RUST_LOG=error cargo run

## CHECK (Run the rust container in check mode)

check-rust:
	docker compose -f $(DOCKER_COMPOSE_DEV) exec rust cargo check

## LINT (Rust code quality checks)

lint-rust:
	docker compose -f $(DOCKER_COMPOSE_DEV) exec rust cargo clippy -- -D warnings

fmt-rust:
	docker compose -f $(DOCKER_COMPOSE_DEV) exec rust cargo fmt

fmt-check-rust:
	docker compose -f $(DOCKER_COMPOSE_DEV) exec rust cargo fmt --check

## UP (Create and start all containers and images)
up:
	make up-rust && make up-vuejs

up-vuejs:
	docker compose -f $(DOCKER_COMPOSE_DEV) watch vuejs
	
up-rust:
	docker compose -f $(DOCKER_COMPOSE_DEV) up rust -d --remove-orphans 
	-docker compose -f $(DOCKER_COMPOSE_DEV) exec rust cargo build --release

## DOWN (Stop the container & image )
down:
	docker compose -f $(DOCKER_COMPOSE_DEV) down

down-rust:
	docker compose -f $(DOCKER_COMPOSE_DEV) down rust

down-vuejs:
	docker compose -f $(DOCKER_COMPOSE_DEV) down vuejs

## SH (Access an interactive shell inside the containers)
sh-rust:
	docker compose -f $(DOCKER_COMPOSE_DEV) exec -it rust sh

sh-vuejs:
	docker compose -f $(DOCKER_COMPOSE_DEV) exec -it vuejs sh


## DELETE (Stop and remove X container and its image)
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


## PROFD TODO
prod: build-dev
	docker build -f rust/.Docker/Dockerfile.prod -t novasound-rust-prod rust
	docker run -d -p 3000:3000 --name novasound-prod novasound-rust-prod
	
## Importe SQL	
import-sql:
	@read -p "Chemin du fichier SQL à importer: (backend/rust/data/example/example_database.sql) " sqlfile; \
	docker compose -f $(DOCKER_COMPOSE_DEV) cp $$sqlfile rust:/tmp/import.sql && \
	docker compose -f $(DOCKER_COMPOSE_DEV) exec rust sh -c "sqlite3 data/database.db < /tmp/import.sql && echo 'Import réussi!'" && \
	docker compose -f $(DOCKER_COMPOSE_DEV) exec rust rm /tmp/import.sql
	


## VERSIONS (Sync all versions from .tools/VERSIONS)

sync-versions:
	sh .tools/sync-versions.sh

sync-versions-dry:
	sh .tools/sync-versions.sh --dry-run


## HOOKS

install-hooks:
	cp .tools/pre-commit.sh .git/hooks/pre-commit
	chmod +x .git/hooks/pre-commit
	@echo "Pre-commit hook installed."


## Clean git checkout 
clean-git:
git branch -vv | grep ': gone]' | awk '{print $1}' | xargs git branch -D


## Update Backend
update-backend:
	cd backend && \
	git checkout main && \
	git pull origin main && \
	cd .. && \
	git add backend && \
