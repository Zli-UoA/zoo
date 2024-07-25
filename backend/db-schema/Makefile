include .env
export $(shell sed 's/=.*//' .env)

migrate:
	set -a && source .env && set +a && \
	npm i && \
	npx prisma migrate dev --name init

generate-entities:
	sea-orm-cli generate entity \
	-u postgresql://$(DB_USER):$(DB_PASSWORD)@$(DB_HOST):$(DB_PORT)/$(DB_NAME) \
	-o ../src/entities
