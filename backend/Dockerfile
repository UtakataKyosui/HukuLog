FROM rust:1.82-slim-bookworm AS builder

RUN apt-get update -y && \
	apt-get upgrade -y &&  \
	apt-get install -y libssl-dev pkg-config

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY migration ./migration

COPY src ./src
RUN cargo build --release

RUN cargo build --release

FROM debian:bookworm-slim AS runtime

RUN apt-get update -y && \
    apt-get install -y libssl3

WORKDIR /app

COPY --from=builder /app/target/release/backend-cli ./
COPY config ./config  

EXPOSE 5151

CMD ["./backend-cli","start"]