DOCKER_COMPOSE_DEV=.tools/docker_compose_dev.yml 
DOCKERFILE_DEV_RUST=rust/.Docker/Dockerfile.dev

## UP (Create and start all containers and images)
up:
	make up-rust && make up-vuejs

up-vuejs:
	docker compose -f $(DOCKER_COMPOSE_DEV) watch vuejs
	
up-rust:
	docker compose -f $(DOCKER_COMPOSE_DEV) up rust -d --remove-orphans 
	docker compose -f $(DOCKER_COMPOSE_DEV) exec rust cargo build --release

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

## PROFD TODO
prod: build-dev
	docker build -f rust/.Docker/Dockerfile.prod -t novasound-rust-prod rust
	docker run -d -p 3000:3000 --name novasound-prod novasound-rust-prod