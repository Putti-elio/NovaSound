DOCKER_COMPOSE_DEV=.tools/docker_compose_dev.yml 
DOCKERFILE_DEV_RUST=backend/.Docker/Dockerfile.dev

up:
	make up-back && make up-front

up-front:
	docker compose -f $(DOCKER_COMPOSE_DEV) watch frontend
	
up-back:
	docker compose -f $(DOCKER_COMPOSE_DEV) up backend -d --remove-orphans
	docker compose -f $(DOCKER_COMPOSE_DEV) exec backend cargo build --release

down:
	docker compose -f $(DOCKER_COMPOSE_DEV) down

delete-rust:
	docker rmi -f rust
 

sh-back:
	docker compose -f $(DOCKER_COMPOSE_DEV) exec -it backend sh

sh-front:
	docker compose -f $(DOCKER_COMPOSE_DEV) exec -it frontend sh

build-front:
	docker compose -f $(DOCKER_COMPOSE_DEV) down
	docker compose -f $(DOCKER_COMPOSE_DEV) build --no-cache
	docker compose -f $(DOCKER_COMPOSE_DEV) watch frontend

build-back:
	docker compose -f $(DOCKER_COMPOSE_DEV) down
	docker compose -f $(DOCKER_COMPOSE_DEV) build --no-cache
	docker compose -f $(DOCKER_COMPOSE_DEV) watch frontend

prod: build-dev
	docker build -f backend/.Docker/Dockerfile.prod -t novasound-backend-prod backend
	docker run -d -p 3000:3000 --name novasound-prod novasound-backend-prod