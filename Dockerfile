FROM rust:latest AS builder
WORKDIR /usr/src/service
COPY . .
RUN cargo install --path .

FROM bitnami/minideb:buster
LABEL maintainer="barlog@tanelorn.li"

COPY --from=builder /usr/local/cargo/bin/service /usr/local/bin/

ENV RUST_LOG axum=debug,tower_http=debug,debug

CMD ["/usr/local/bin/service"]
