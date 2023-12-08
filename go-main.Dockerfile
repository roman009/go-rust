FROM golang:1.21 as builder

WORKDIR /build
COPY * /build/
RUN CGO_ENABLED=0 GOOS=linux go build go-main.go

FROM alpine:latest
WORKDIR /app
COPY --from=builder /build/go-main /app/

EXPOSE 8085

ENTRYPOINT ["/app/go-main"]