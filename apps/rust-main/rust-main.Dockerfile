FROM docker.io/library/rust:1.74.0-slim-bullseye as builder

WORKDIR /build
COPY Cargo.toml /build/
COPY Cargo.lock /build/
RUN cargo fetch
COPY src /build/src
RUN cargo build --release --jobs 2

FROM docker.io/library/debian:bullseye-slim

WORKDIR /app
RUN apt update -y && apt install -y libc6 libssl-dev build-essential

COPY --from=builder /build/target/release/rust-main /app/
ARG LISTENING_PORT=8080
ENV LISTENING_PORT=${LISTENING_PORT}
EXPOSE ${LISTENING_PORT}

ENTRYPOINT ["/app/rust-main"]