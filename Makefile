DOCKER_COMPOSE_DEV=.tools/docker_compose_dev.yml

## @category Fix

## @description Auto-fix Rust backend issues
## @depends up-backend
fix-backend:
	docker compose -f $(DOCKER_COMPOSE_DEV) exec backend cargo fix -p rust

## @category Run

## @description Run backend in release mode
run-backend-release:
	docker compose -f $(DOCKER_COMPOSE_DEV) exec backend cargo run --release

## @description Run backend in development mode
run-backend:
	docker compose -f $(DOCKER_COMPOSE_DEV) exec backend cargo run 

## @description Run backend with error logging only
run-backend-error:
	docker compose -f $(DOCKER_COMPOSE_DEV) exec backend RUST_LOG=error cargo run

## @category Check

## @description Check Rust backend code (fast compilation)
## @depends up-backend
check-backend:
	docker compose -f $(DOCKER_COMPOSE_DEV) exec backend cargo check

## @category Lint

## @description Lint Rust backend code with Clippy
## @depends up-backend
lint-backend:
	docker compose -f $(DOCKER_COMPOSE_DEV) exec -T backend cargo clippy -- -D warnings

## @description Format Rust backend code
## @depends up-backend
fmt-backend:
	docker compose -f $(DOCKER_COMPOSE_DEV) exec -T backend cargo fmt

## @description Check Rust backend code formatting
## @depends up-backend
fmt-check-backend:
	docker compose -f $(DOCKER_COMPOSE_DEV) exec -T backend cargo fmt --check

## @category Up

## @description Start all services (backend + frontend)
up:
	make up-backend && make up-vuejs

## @description Start frontend with hot-reload
up-vuejs:
	docker compose -f $(DOCKER_COMPOSE_DEV) watch vuejs
	
## @description Start backend service
## @depends check-backend
up-backend:
	docker compose -f $(DOCKER_COMPOSE_DEV) up backend -d --remove-orphans 
	-docker compose -f $(DOCKER_COMPOSE_DEV) exec backend cargo build --release

## @category Down

## @description Stop all services
down:
	docker compose -f $(DOCKER_COMPOSE_DEV) down

## @description Stop backend service
down-backend:
	docker compose -f $(DOCKER_COMPOSE_DEV) down backend

## @description Stop frontend service
down-vuejs:
	docker compose -f $(DOCKER_COMPOSE_DEV) down vuejs

## @category Shell

## @description Open backend container shell
sh-backend:
	docker compose -f $(DOCKER_COMPOSE_DEV) exec -it backend sh

## @description Open frontend container shell
sh-vuejs:
	docker compose -f $(DOCKER_COMPOSE_DEV) exec -it vuejs sh

## @category Delete

## @description Delete backend container and image
delete-backend:
	make down-backend
	docker compose -f $(DOCKER_COMPOSE_DEV) rm -f backend
	docker image rm -f tools-backend

## @description Delete frontend container and image
delete-vuejs:
	make down-vuejs
	docker compose -f $(DOCKER_COMPOSE_DEV) rm -f vuejs
	docker image rm -f tools-vuejs

## @description Delete all containers and images
delete-all:
	make delete-backend 
	make delete-vuejs

## @category Test

## @description Run backend tests
## @depends up-backend
test: 
	docker compose -f .tools/docker_compose_dev.yml exec -T backend cargo test

## @category Prod

## @description Build and run backend in production mode
prod:
	docker build -f .docker/backend/Dockerfile.prod -t novasound-backend-prod backend
	docker run -d -p 3000:3000 --name novasound-prod novasound-backend-prod
	
## @category Database

## @description Import SQL file into database
## @env sqlfile
import-sql:
	@read -p "Chemin du fichier SQL à importer: (backend/data/example/example_database.sql) " sqlfile; \
	docker compose -f $(DOCKER_COMPOSE_DEV) cp $$sqlfile backend:/tmp/import.sql && \
	docker compose -f $(DOCKER_COMPOSE_DEV) exec backend sh -c "sqlite3 data/database.db < /tmp/import.sql && echo 'Import réussi!'" && \
	docker compose -f $(DOCKER_COMPOSE_DEV) exec backend rm /tmp/import.sql
	
## @category Clean

## @description Clean deleted git branches
clean-git:
	git branch -vv | grep ': gone]' | awk '{print $1}' | xargs git branch -D
