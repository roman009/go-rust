FROM golang:1.21 as builder

WORKDIR /build
COPY * /build/
RUN CGO_ENABLED=0 GOOS=linux go build main.go

FROM alpine:latest
WORKDIR /app
COPY --from=builder /build/main /app/

EXPOSE 8085

ENTRYPOINT ["/app/main"]