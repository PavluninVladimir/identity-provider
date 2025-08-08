# Multi-stage Dockerfile using Alpine
FROM rust:1-alpine AS builder
WORKDIR /usr/src/app
# Install build dependencies
RUN apk add --no-cache musl-dev openssl-dev pkgconfig
# Copy source
COPY . .
# Build release binary
RUN cargo build --release

FROM alpine:3.19
RUN apk add --no-cache ca-certificates
WORKDIR /usr/local/bin
COPY --from=builder /usr/src/app/target/release/identity-provider .
CMD ["./identity-provider"]
