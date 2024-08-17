.PHONY: migrate generate-entities

migrate:
	$(MAKE) -C backend migrate

generate-entities:
	$(MAKE) -C backend generate-entities

db-up:
	docker run \
		-d \
		--rm \
		--name zoo-postgres \
		--env POSTGRES_USER=postgres \
		--env POSTGRES_PASSWORD=postgres \
		--env POSTGRES_DB=db \
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

