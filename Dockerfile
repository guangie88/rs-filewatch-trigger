FROM clux/muslrust:stable as builder

WORKDIR /app
COPY ./Cargo.toml ./Cargo.lock ./
COPY ./src ./src

RUN set -x \
    && cargo fetch --locked -v \
    && cargo build --release

FROM alpine:3.8

COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/fwt /usr/local/bin/
