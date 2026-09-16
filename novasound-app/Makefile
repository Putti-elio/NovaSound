DOCKER_COMPOSE = docker compose -f project/docker/compose.yml
WIFI_IP := $(shell ip -4 -o addr show scope global | awk '$$2 ~ /^wl/ {sub(/\/.*/, "", $$4); print $$4; exit}')
TAURI_PID_FILE := /tmp/novasound-app-tauri.pid
TAURI_LOG_FILE := /tmp/novasound-app-tauri.log
export WIFI_IP

define PRINT_DEV_URLS
	@if [ -n "$(WIFI_IP)" ]; then printf "NovaSound app (Wi-Fi):  http://%s:5173\n" "$(WIFI_IP)"; else printf "NovaSound app (local):   http://localhost:5173\n"; fi
endef

.PHONY: dev dev-docker docker-down web tauri up down restart clear build build-tauri

## @category Development

## @description Start the Vite development server
dev:
	bun install --frozen-lockfile
	bun run dev

## @description Start Vite in a Bun container (no host Bun installation required)
dev-docker:
	$(DOCKER_COMPOSE) run --rm --service-ports app bun install --frozen-lockfile
	$(PRINT_DEV_URLS)
	$(DOCKER_COMPOSE) up app

docker-down:
	$(MAKE) down

## @category Docker

## @description Start the web app in Docker
web:
	$(DOCKER_COMPOSE) run --rm app bun install --frozen-lockfile
	$(DOCKER_COMPOSE) up -d --build app
	$(PRINT_DEV_URLS)

## @description Start the Tauri desktop app
tauri: web
	@if [ -f "$(TAURI_PID_FILE)" ] && kill -0 "$$(cat "$(TAURI_PID_FILE)")" 2>/dev/null; then printf "Tauri is already running. Logs: %s\n" "$(TAURI_LOG_FILE)"; exit 0; fi
	@rm -f "$(TAURI_PID_FILE)"
	@setsid sh -c 'cd code/desktop && exec cargo tauri dev' >"$(TAURI_LOG_FILE)" 2>&1 & echo $$! > "$(TAURI_PID_FILE)"
	@printf "Tauri started. Logs: %s\n" "$(TAURI_LOG_FILE)"

## @description Start the web and Tauri apps
up: web tauri

## @description Stop the web and Tauri apps
down:
	@if [ -f "$(TAURI_PID_FILE)" ]; then pid="$$(cat "$(TAURI_PID_FILE)")"; kill -- -"$$pid" 2>/dev/null || true; rm -f "$(TAURI_PID_FILE)"; fi
	$(DOCKER_COMPOSE) down

## @description Restart the web and Tauri apps
restart: down up

## @description Remove the web and Tauri apps plus Docker dependencies
clear: down
	$(DOCKER_COMPOSE) down --volumes --rmi local

## @category Build

## @description Build browser assets
build:
	bun install --frozen-lockfile
	bun run build

## @description Build Linux desktop packages
## @depends build
build-tauri: build
	cd code/desktop && NO_STRIP=1 cargo tauri build
