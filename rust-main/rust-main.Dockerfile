FROM docker.io/library/rust:1.74.0-slim-bullseye as builder

WORKDIR /build
COPY Cargo.toml /build/
COPY Cargo.lock /build/
COPY src /build/src
RUN cargo fetch
RUN cargo build --release

FROM docker.io/library/debian:bullseye-slim
WORKDIR /app
COPY --from=builder /build/target/release/rust-main /app/
RUN apt update -y && apt install -y libc6 libssl-dev build-essential

EXPOSE 8084

ENTRYPOINT ["/app/rust-main"]