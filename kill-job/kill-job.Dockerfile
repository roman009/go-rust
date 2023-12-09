FROM golang:1.21 as builder

WORKDIR /build
COPY * /build/
RUN CGO_ENABLED=0 GOOS=linux go build main.go

FROM alpine:latest
WORKDIR /app
COPY --from=builder /build/main /app/

ARG RUST_MAIN_APP_URL=http://rust-main.default.svc.cluster.local
ARG GO_MAIN_APP_URL=http://go-main.default.svc.cluster.local
ENV RUST_MAIN_APP_URL=$RUST_MAIN_APP_URL
ENV GO_MAIN_APP_URL=$GO_MAIN_APP_URL

ENTRYPOINT ["/app/main"]