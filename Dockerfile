FROM rust:latest as build

WORKDIR /usr/src/app

COPY . .

RUN cargo build --release

FROM debian:12.1-slim

WORKDIR /usr/src/app

COPY --from=build /usr/src/app/target/release/rabbit-locker .

EXPOSE 8000

CMD ["./rabbit-locker"]
