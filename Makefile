include .env

.PHONY: docker-up
docker-up:
	docker compose up -d

.PHONY: docker-down
docker-up:
	docker compose down

.PHONY: migrate
migrate:
	DATABASE_URL=${DATABASE_URL} refinery migrate -e DATABASE_URL -p ./migration

.PHONY: build
build:
	@cargo build --release

.PHONY: watch
watch:
	@cargo watch -x run

.PHONY: start
start:
	./target/release/dapp-lab-server-proto

.PHONY: schedule
schedule:
	@cargo run --example schedule
