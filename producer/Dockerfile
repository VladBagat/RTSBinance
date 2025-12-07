FROM rust:alpine AS builder
WORKDIR /producer
RUN apk add --no-cache build-base cmake pkgconf openssl-dev cyrus-sasl-dev zlib-dev zlib-static
COPY --from=shared . /producer/shared
COPY . .
RUN cargo install --path .

FROM alpine:3.18
COPY --from=builder /usr/local/cargo/bin/producer /usr/local/bin/producer
CMD ["producer"]