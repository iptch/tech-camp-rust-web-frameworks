FROM rust:1.62.1-buster AS builder

WORKDIR /app

RUN rustup toolchain install nightly && \
    rustup default nightly

COPY ./Cargo.toml ./Cargo.lock ./
COPY ./src src

# build with x86_64-unknown-linux-musl to make it runs on alpine.
RUN cargo build --release

# runtime image
FROM debian:buster-slim

COPY ./Rocket.toml /Rocket.toml
COPY --from=builder /app/target/release/ho-erfa-sample /app/bin/ho-erfa-sample
CMD ["/app/bin/ho-erfa-sample"]
