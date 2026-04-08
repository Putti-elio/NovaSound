DOCKER_DIR := setup/docker
ENV_FILE := $(DOCKER_DIR)/.env
COMPOSE_FILES := -f $(DOCKER_DIR)/compose.yml

ifeq ($(APP_ENV),dev)
    COMPOSE_FILES += -f $(DOCKER_DIR)/docker-compose.dev.yml
else ifeq ($(APP_ENV),prod)
    COMPOSE_FILES += -f $(DOCKER_DIR)/docker-compose.prod.yml
endif

COMPOSE := docker compose \
	$(COMPOSE_FILES) \
	--env-file $(DOCKER_DIR)/images/versions \
	--env-file $(ENV_FILE) \
	--project-directory .

## @category Fix

## @description Auto-fix Rust backend issues
## @depends up-backend
fix-backend:
	$(COMPOSE) exec backend cargo fix -p rust

## @category Run

## @description Run backend in release mode
run-backend-release:
	$(COMPOSE) exec backend cargo run --release

## @description Run backend in development mode
run-backend:
	$(COMPOSE) exec backend cargo run 

## @description Run backend with error logging only
run-backend-error:
	$(COMPOSE) exec backend RUST_LOG=error cargo run

## @category Check

## @description Check Rust backend code (fast compilation)
## @depends up-backend
check-backend:
	$(COMPOSE) exec backend cargo check

## @category Lint

## @description Lint Rust backend code with Clippy
## @depends up-backend
lint-backend:
	$(COMPOSE) exec -T backend cargo clippy -- -D warnings

## @description Format Rust backend code
## @depends up-backend
fmt-backend:
	$(COMPOSE) exec -T backend cargo fmt

## @description Check Rust backend code formatting
## @depends up-backend
fmt-check-backend:
	$(COMPOSE) exec -T backend cargo fmt --check

## @category Up

## @description Start all services (backend + frontend)
up: up-backend up-vuejs

## @description Start frontend with hot-reload
up-vuejs:
	$(COMPOSE) up vuejs -d
	
## @description Start backend service
## @depends check-backend
up-backend:
	$(COMPOSE) up backend -d --remove-orphans 
	-$(COMPOSE) exec backend cargo build --release

## @category Down

## @description Stop all services
down:
	$(COMPOSE) down

## @description Stop backend service
down-backend:
	$(COMPOSE) down backend

## @description Stop frontend service
down-vuejs:
	$(COMPOSE) down vuejs

## @category Shell

## @description Open backend container shell
sh-backend:
	$(COMPOSE) exec -it backend sh

## @description Open frontend container shell
sh-vuejs:
	$(COMPOSE) exec -it vuejs sh

## @category Delete

## @description Delete backend container and image
delete-backend:
	make down-backend
	$(COMPOSE) rm -f backend
	docker image rm -f tools-backend

## @description Delete frontend container and image
delete-vuejs:
	make down-vuejs
	$(COMPOSE) rm -f vuejs
	docker image rm -f tools-vuejs

## @description Delete all containers and images
delete-all:
	make delete-backend 
	make delete-vuejs

## @category Test

## @description Run backend tests
## @depends up-backend
test: 
	$(COMPOSE) exec -T backend cargo test

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
	$(COMPOSE) cp $$sqlfile backend:/tmp/import.sql && \
	$(COMPOSE) exec backend sh -c "sqlite3 data/database.db < /tmp/import.sql && echo 'Import réussi!'" && \
	$(COMPOSE) exec backend rm /tmp/import.sql
	
## @category Clean

## @description Clean deleted git branches
clean-git:
	git branch -vv | grep ': gone]' | awk '{print $1}' | xargs git branch -D
