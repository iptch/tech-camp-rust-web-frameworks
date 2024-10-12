FROM rust:1

WORKDIR /usr/src/myapp
COPY . .

RUN cargo install --path .

CMD ["rocket-text-searcher"]

