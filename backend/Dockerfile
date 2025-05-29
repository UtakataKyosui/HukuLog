FROM rust:1.79-slim-bookworm AS builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY migration ./migration

RUN cargo build --release

COPY src ./src

RUN cargo build --release

FROM debian:bookworm-slim AS runtime

WORKDIR /app

COPY --from=builder /app/target/release/backend-cli ./

EXPOSE 8000



CMD ["./backend-cli"]\n