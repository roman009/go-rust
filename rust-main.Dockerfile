FROM docker.io/library/rust:1.74.0-slim-bullseye as builder

WORKDIR /build
COPY * /build/
RUN cargo build --release

FROM docker.io/library/debian:bullseye-slim
WORKDIR /app
COPY --from=builder /build/target/release/rust-main /app/
RUN apt update -y && apt upgrade -y && apt install -y libc6 libssl-dev build-essential curl

EXPOSE 8084

ENTRYPOINT ["/app/rust-main"]