.PHONY: migrate generate-entities db-up db-down db-restart db-reset

migrate:
	$(MAKE) -C backend migrate

generate-entities:
	$(MAKE) -C backend generate-entities

db-up:
	$(MAKE) -C infra db-up

db-down:
	$(MAKE) -C infra db-down

db-restart:
	$(MAKE) -C infra db-restart

db-reset:
	$(MAKE) -C infra db-reset