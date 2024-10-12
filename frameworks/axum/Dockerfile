FROM rust:1 as builder

WORKDIR /usr/src/myapp
COPY Cargo.lock Cargo.lock
COPY Cargo.toml Cargo.toml
COPY src/ src/

RUN cargo install --path .

FROM debian:stable
COPY --from=builder /usr/local/cargo/bin/novelty /usr/local/bin/novelty
CMD ["novelty"]
