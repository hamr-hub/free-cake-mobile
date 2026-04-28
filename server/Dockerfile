FROM rust:latest AS builder

WORKDIR /app

RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

COPY server/Cargo.toml server/Cargo.lock* ./ 
COPY server/src ./src

RUN cargo build --release 2>&1 && mv target/release/free-cake-server /tmp/free-cake-server-bin

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates libssl3 curl && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /tmp/free-cake-server-bin /app/free-cake-server
COPY server/src/db/migrations ./migrations

EXPOSE 3000

ENV RUST_LOG=info
ENV SERVER_PORT=3000

ENTRYPOINT ["/app/free-cake-server"]
