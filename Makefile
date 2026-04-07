DOCKER_COMPOSE_DEV=.tools/docker_compose_dev.yml

fix-backend:
	docker compose -f $(DOCKER_COMPOSE_DEV) exec backend cargo fix -p rust

## RUN

run-backend-release:
	docker compose -f $(DOCKER_COMPOSE_DEV) exec backend cargo run --release

run-backend:
	docker compose -f $(DOCKER_COMPOSE_DEV) exec backend cargo run 

run-backend-error:
	docker compose -f $(DOCKER_COMPOSE_DEV) exec backend RUST_LOG=error cargo run

## CHECK

check-backend:
	docker compose -f $(DOCKER_COMPOSE_DEV) exec backend cargo check

## LINT

lint-backend:
	docker compose -f $(DOCKER_COMPOSE_DEV) exec -T backend cargo clippy -- -D warnings

fmt-backend:
	docker compose -f $(DOCKER_COMPOSE_DEV) exec -T backend cargo fmt

fmt-check-backend:
	docker compose -f $(DOCKER_COMPOSE_DEV) exec -T backend cargo fmt --check

## UP

up:
	make up-backend && make up-vuejs
	
up-vuejs:
	docker compose -f $(DOCKER_COMPOSE_DEV) watch vuejs
	
up-backend:
	docker compose -f $(DOCKER_COMPOSE_DEV) up backend -d --remove-orphans 
	-docker compose -f $(DOCKER_COMPOSE_DEV) exec backend cargo build --release

## DOWN

down:
	docker compose -f $(DOCKER_COMPOSE_DEV) down

down-backend:
	docker compose -f $(DOCKER_COMPOSE_DEV) down backend

down-vuejs:
	docker compose -f $(DOCKER_COMPOSE_DEV) down vuejs

## SH

sh-backend:
	docker compose -f $(DOCKER_COMPOSE_DEV) exec -it backend sh

sh-vuejs:
	docker compose -f $(DOCKER_COMPOSE_DEV) exec -it vuejs sh

## DELETE

delete-backend:
	make down-backend
	docker compose -f $(DOCKER_COMPOSE_DEV) rm -f backend
	docker image rm -f tools-backend

delete-vuejs:
	make down-vuejs
	docker compose -f $(DOCKER_COMPOSE_DEV) rm -f vuejs
	docker image rm -f tools-vuejs

delete-all:
	make delete-backend 
	make delete-vuejs

## TEST

test: 
	docker compose -f .tools/docker_compose_dev.yml exec -T backend cargo test

## PROD

prod:
	docker build -f .docker/backend/Dockerfile.prod -t novasound-backend-prod backend
	docker run -d -p 3000:3000 --name novasound-prod novasound-backend-prod
	
## Import SQL

import-sql:
	@read -p "Chemin du fichier SQL à importer: (backend/data/example/example_database.sql) " sqlfile; \
	docker compose -f $(DOCKER_COMPOSE_DEV) cp $$sqlfile backend:/tmp/import.sql && \
	docker compose -f $(DOCKER_COMPOSE_DEV) exec backend sh -c "sqlite3 data/database.db < /tmp/import.sql && echo 'Import réussi!'" && \
	docker compose -f $(DOCKER_COMPOSE_DEV) exec backend rm /tmp/import.sql
	

## CLEAN

clean-git:
	git branch -vv | grep ': gone]' | awk '{print $1}' | xargs git branch -D
