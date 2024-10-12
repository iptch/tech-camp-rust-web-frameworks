FROM rust:1 AS builder

WORKDIR /app

#RUN rustup toolchain install nightly && \
#    rustup default nightly

COPY ./Cargo.toml ./Cargo.lock ./
COPY ./src src

# build with x86_64-unknown-linux-musl to make it runs on alpine.
RUN cargo build --release

# runtime image
FROM debian:buster-slim

COPY --from=builder /app/target/release/actix-test /app/bin/actix-test
CMD ["/app/bin/actix-test"]
