.PHONY: run dev build docker-build docker-run docker-stop docker-clean test check fmt clippy doc

# ─── Development ───

run:
	RUST_LOG=debug cargo run

dev:
	RUST_LOG=debug cargo watch -x run

build:
	cargo build --release

check:
	cargo check

test:
	cargo test -- --test-threads=1

fmt:
	cargo fmt

clippy:
	cargo clippy -- -D warnings

doc:
	cargo doc --open

# ─── Docker ───

docker-build:
	docker compose build

docker-run:
	docker compose up -d

docker-stop:
	docker compose stop

docker-clean:
	docker compose down --rmi all --volumes

docker-logs:
	docker compose logs -f api
