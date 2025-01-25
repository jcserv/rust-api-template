include .env
export

.PHONY: build run run-watch dev-db dev-db-down

build:
	cargo build

run:
	cargo run

run-watch:
	cargo watch -q -c -w src/ -x run

dev-db:
	docker compose -p rust-api-template -f docker-compose.yml up --detach

dev-db-down:
	docker compose -p rust-api-template -f docker-compose.yml down -v

migrate:
	sqlx migrate run