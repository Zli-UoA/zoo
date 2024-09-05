PHONY: db-up db-down db-restart db-reset

include ../backend/.env

db-up:
	docker run \
		-d \
		--rm \
		--name zoo-postgres \
		--env POSTGRES_USER=$(DB_USER) \
		--env POSTGRES_PASSWORD=$(DB_PASSWORD) \
		--env POSTGRES_DB=$(DB_NAME) \
		-p 5432:5432 \
		--mount type=volume,source=zoo-postgres-data,target=/var/lib/postgresql/data \
		postgres:16

db-down:
	docker stop zoo-postgres

db-restart:
	docker restart zoo-postgres

db-reset:
	docker rm -f zoo-postgres
	docker volume rm zoo-postgres-data
