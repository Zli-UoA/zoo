.PHONY: migrate generate-entity

migrate:
	$(MAKE) -C backend migrate

generate-entity:
	$(MAKE) -C backend generate-entity
