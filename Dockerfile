FROM rust:1.65.0 as fetcher

WORKDIR /cargo

COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock

FROM rust:1.65.0 as builder

WORKDIR /build

COPY ./src /build/src
COPY ./Cargo.toml /build/Cargo.toml

RUN cargo build --release

FROM rust:1.65.0 as runner

WORKDIR /app

COPY --from=builder /build/target/release/ /app

ENV RUST_LOG=${RUST_LOG}
ENV TELOXIDE_TOKEN=${TELOXIDE_TOKEN}

ARG EXPOSE_PORT=${PORT}

EXPOSE ${EXPOSE_PORT}

ENTRYPOINT ["/app/swastika-bot"]