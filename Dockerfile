FROM rust:1.65.0 as fetcher

WORKDIR /cargo

COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock

RUN cargo fetch

FROM rust:1.65.0 as builder

WORKDIR /build

COPY --from=fetcher /cargo/target ./target
COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock
COPY ./src /build/src

RUN cargo build --release

FROM rust:1.65.0 as runner

WORKDIR /app

COPY --from=builder /build/target/release/ /app

ENV TELOXIDE_TOKEN=${TELOXIDE_TOKEN}
ENV RUST_LOG=${RUST_LOG}
ENV PORT=${PORT}
ENV DOMAIN=${DOMAIN}
ENV POLLING_MODE=${POLLING_MODE}

ARG EXPOSE_PORT=${PORT}

EXPOSE ${EXPOSE_PORT}

ENTRYPOINT ["/app/swastika-bot"]