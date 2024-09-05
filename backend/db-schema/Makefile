include ../.env
export $(shell sed 's/=.*//' ../.env)

migrate:
	set -a && source ../.env && set +a && \
	npx prisma migrate dev --skip-generate

generate-entities:
	sea-orm-cli generate entity \
	-u postgresql://$(DB_USER):$(DB_PASSWORD)@$(DB_HOST):$(DB_PORT)/$(DB_NAME) \
	-o ../src/generate/entities
