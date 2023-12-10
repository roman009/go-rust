FROM docker.io/library/rust:1.74.0-slim-bullseye as builder

WORKDIR /build
COPY Cargo.toml /build/
COPY Cargo.lock /build/
RUN cargo fetch
COPY src /build/src
RUN cargo build --release

FROM docker.io/library/debian:bullseye-slim
WORKDIR /app
COPY --from=builder /build/target/release/service-discoverer /app/
RUN apt update -y && apt install -y libc6 libssl-dev build-essential

ARG LISTENING_PORT=8080
ENV LISTENING_PORT=${LISTENING_PORT}
EXPOSE ${LISTENING_PORT}

ENTRYPOINT ["/app/service-discoverer"]