FROM rust:1-slim as builder

WORKDIR /usr/src/date-group

COPY ./src /usr/src/date-group/src
COPY Cargo.toml /usr/src/date-group/Cargo.toml
COPY Cargo.lock /usr/src/date-group/Cargo.lock

RUN cargo build --release

FROM debian:buster-slim

COPY --from=builder /usr/src/date-group/target/release/date-group /usr/local/bin/date-group

CMD ["/usr/local/bin/date-group"]
