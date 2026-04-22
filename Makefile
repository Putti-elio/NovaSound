DOCKER_DIR := setup/docker
ENV_FILE := $(DOCKER_DIR)/.env

# Read APP_ENV from .env file if not set via command line
ifeq ($(origin APP_ENV),undefined)
    _APP_ENV := $(shell grep -E '^APP_ENV=' $(ENV_FILE) 2>/dev/null | cut -d= -f2)
    ifneq ($(_APP_ENV),)
        APP_ENV := $(_APP_ENV)
    else
        APP_ENV := dev
    endif
endif

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
	@printf "Frontend Docker stack is disabled for now.\n"
	
## @description Start backend service
## @depends check-backend
up-backend:
	$(COMPOSE) up backend -d --remove-orphans 

## @category Down

## @description Stop all services
down:
	$(COMPOSE) down

## @description Stop backend service
down-backend:
	$(COMPOSE) down backend

## @description Stop frontend service
down-vuejs:
	@printf "Frontend Docker stack is disabled for now.\n"

## @category Shell

## @description Open backend container shell
sh-backend:
	$(COMPOSE) exec -it backend sh

## @description Open frontend container shell
sh-vuejs:
	@printf "Frontend Docker stack is disabled for now.\n"

## @category Delete

## @description Delete backend container and image
clear-backend:
	$(COMPOSE) down backend
	-docker image rm -f novasound-backend
	-docker image rm -f novasound/backend:latest

## @description Delete frontend container and image
clear-vuejs:
	@printf "Frontend Docker stack is disabled for now.\n"

## @description Delete all containers and images
clear:
	make clear-backend 
	make clear-vuejs

## @category Test

## @description Run backend tests
## @depends up-backend
test: 
	$(COMPOSE) exec -T backend cargo test

## @category Prod

## @description Build and tag backend production image
build-prod:
	@if [ -n "$$(git describe --tags --abbrev=0 2>/dev/null)" ]; then \
		TAG=$$(git describe --tags --abbrev=0); \
		echo "Building with tag: $$TAG"; \
		TAG=$$TAG docker compose -f $(DOCKER_DIR)/compose.yml -f $(DOCKER_DIR)/docker-compose.prod.yml --project-directory . build backend; \
	else \
		echo "No git tags found, building with latest"; \
		docker compose -f $(DOCKER_DIR)/compose.yml -f $(DOCKER_DIR)/docker-compose.prod.yml --project-directory . build backend; \
	fi

## @description Build, tag, and run backend in production mode
prod: build-prod
	APP_ENV=prod docker compose -f $(DOCKER_DIR)/compose.yml -f $(DOCKER_DIR)/docker-compose.prod.yml --project-directory . up -d backend

## @description Create a version tag (use TAG=v1.2.3 make tag)
tag:
	@if [ -z "$(TAG)" ]; then \
		echo "Usage: make tag TAG=v1.2.3"; \
		exit 1; \
	fi
	git tag -a $(TAG) -m "Release $(TAG)"
	@echo "Tag $(TAG) created. Push with: git push origin $(TAG)"
	
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
