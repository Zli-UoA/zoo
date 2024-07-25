.PHONY: migrate generate-entities

migrate:
	$(MAKE) -C backend migrate

generate-entities:
	$(MAKE) -C backend generate-entities
