DOCKER_COMPOSE = docker compose -f project/docker/compose.yml

.PHONY: dev dev-docker docker-down build build-tauri

## @category Development

## @description Start the Vite development server
dev:
	bun install --frozen-lockfile
	bun run dev

## @description Start Vite in a Bun container (no host Bun installation required)
dev-docker:
	$(DOCKER_COMPOSE) run --rm --service-ports app bun install --frozen-lockfile
	$(DOCKER_COMPOSE) up app

docker-down:
	$(DOCKER_COMPOSE) down

## @category Build

## @description Build browser assets
build:
	bun install --frozen-lockfile
	bun run build

## @description Build Linux desktop packages
## @depends build
build-tauri: build
	cd code/desktop && NO_STRIP=1 cargo tauri build
