.PHONY: build up down restart logs clean status prune

build:
	docker-compose build

up:
	docker-compose up -d

run: build up

down:
	docker-compose down

restart:
	docker-compose restart

logs:
	docker-compose logs -f

clean:
	docker-compose down -v
	docker rmi telegram-dice-bot 2>/dev/null || true

status:
	docker-compose ps

prune:
	docker system prune -f
