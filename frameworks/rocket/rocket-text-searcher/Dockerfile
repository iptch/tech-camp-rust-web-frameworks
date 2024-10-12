FROM rust:1 as builder

WORKDIR /usr/src/myapp
COPY Cargo.lock Cargo.lock
COPY Cargo.toml Cargo.toml
COPY src/ src/

RUN cargo install --path .

FROM debian:stable
COPY --from=builder /usr/local/cargo/bin/rocket-text-searcher /usr/local/bin/rocket-text-searcher
CMD ["rocket-text-searcher"]
