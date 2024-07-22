include .env
export $(shell sed 's/=.*//' .env)

migrate:
	./migrate.sh

generate-entities:
	sea-orm-cli generate entity \
	-u postgresql://$(DB_USER):$(DB_PASSWORD)@$(DB_HOST):$(DB_PORT)/$(DB_NAME) \
	-o ../src/entities
