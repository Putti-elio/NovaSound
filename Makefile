DOCKER_COMPOSE_DEV=.tools/docker_compose_dev.yml 
DOCKERFILE_DEV_RUST=backend/.Docker/Dockerfile.dev

up:
	docker compose -f $(DOCKER_COMPOSE_DEV) up -d --remove-orphans 

down:
	docker compose -f $(DOCKER_COMPOSE_DEV) down

sh-back:
	docker compose -f $(DOCKER_COMPOSE_DEV) exec -it backend sh

sh-front:
	docker compose -f $(DOCKER_COMPOSE_DEV) exec -it frontend sh

build-dev:
	docker build -f backend/.Docker/Dockerfile.dev -t novasound-backend-dev backend

prod: build-dev
	docker build -f backend/.Docker/Dockerfile.prod -t novasound-backend-prod backend
	docker run -d -p 3000:3000 --name novasound-prod novasound-backend-prod