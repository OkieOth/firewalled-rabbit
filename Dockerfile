# -*- mode: dockerfile -*-

# ------------------------------------------------------------------------------
# Cargo Build Stage
# ------------------------------------------------------------------------------

FROM alpine:latest as builder

# Update the system as needed
RUN apk update

# Install typical system packages we use for building our app.
RUN apk add binutils build-base ca-certificates curl file g++ gcc libressl-dev make patch rust

# Install Rust with typical settings and as a typical user.
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

WORKDIR /usr/src/app

COPY . .

RUN . ~/.cargo/env && RUSTFLAGS="-C target-feature=-crt-static" cargo build --release --target=x86_64-unknown-linux-musl

# ------------------------------------------------------------------------------
# Final Stage
# ------------------------------------------------------------------------------

FROM rabbitmq:3.11.19-management-alpine

RUN apk add --update --no-cache iptables

WORKDIR /opt/rabbit-locker/

COPY --from=builder /usr/src/app/target/x86_64-unknown-linux-musl/release/rabbit-locker .
COPY --from=builder /usr/src/app/res/start.sh /opt/start.sh

EXPOSE 5672 15672 8000

CMD ["/opt/start.sh"]