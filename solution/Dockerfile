FROM lukemathwalker/cargo-chef:latest-rust-1.92-trixie AS chef
COPY ./solution/rust-toolchain.toml rust-toolchain.toml
WORKDIR /usr/src

FROM chef AS planner
COPY ./solution .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder

ENV OPENSSL_STATIC=1

COPY --from=planner /usr/src/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json --bin antifraud-monolyth

COPY ./solution .
RUN SQLX_OFFLINE=true cargo build --release --bin antifraud-monolyth


FROM debian:trixie-slim

RUN apt-get update \
	&& apt-get install -y --no-install-recommends ca-certificates  \
	&& rm -rf /var/lib/apt/lists/*

WORKDIR /app

RUN useradd -MN app
USER app

COPY --from=builder /usr/src/target/release/antifraud-monolyth app

CMD ["/app"]
