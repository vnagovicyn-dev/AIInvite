SHELL := /usr/bin/env bash

.PHONY: help db-init db-start db-stop db-status db-reset db-migrate compose-up compose-down compose-logs compose-ps run check fmt test

help:
	@printf "%s\n" \
		"make db-init    - initialize local PostgreSQL cluster" \
		"make db-start   - start PostgreSQL" \
		"make db-stop    - stop PostgreSQL" \
		"make db-status  - show PostgreSQL status" \
		"make db-reset   - recreate PostgreSQL data directory" \
		"make db-migrate - apply embedded SQLx migrations" \
		"make compose-up - start Docker infrastructure" \
		"make compose-down - stop Docker infrastructure" \
		"make compose-logs - tail Docker logs" \
		"make compose-ps - list Docker services" \
		"make run        - run Rust application" \
		"make check      - cargo check" \
		"make fmt        - cargo fmt" \
		"make test       - cargo test"

db-init:
	@./scripts/dev-db.sh init

db-start:
	@./scripts/dev-db.sh start

db-stop:
	@./scripts/dev-db.sh stop

db-status:
	@./scripts/dev-db.sh status

db-reset:
	@./scripts/dev-db.sh reset

db-migrate:
	@cargo run -- migrate

compose-up:
	@cargo build --release
	@docker compose up -d --build

compose-down:
	@docker compose down

compose-logs:
	@docker compose logs -f --tail=100

compose-ps:
	@docker compose ps

run:
	@cargo run -- serve

check:
	@cargo check

fmt:
	@cargo fmt

test:
	@cargo test
