FROM rust:1.65.0 as fetcher

WORKDIR /cargo

COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock

FROM rust:1.65.0 as builder

WORKDIR /build

COPY ./src /build/src
COPY ./Cargo.toml /build/Cargo.toml
COPY ./Cargo.lock /build/Cargo.lock

RUN cargo build --release

FROM rust:1.65.0 as runner

WORKDIR /app

ENV RUST_LOG=${RUST_LOG}
ENV TELOXIDE_TOKEN=${TELOXIDE_TOKEN}

COPY --from=builder /build/target/release/ /app

ENTRYPOINT ["/app/swastika-bot"]