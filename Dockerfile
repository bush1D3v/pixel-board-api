FROM rust:latest AS builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo fetch

COPY src/ ./src/
RUN cargo build --release

# ─── Runtime ───
FROM debian:trixie-slim

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /app/target/release/pixel_board_api .

EXPOSE 8080

CMD ["./pixel_board_api"]
