FROM rust:slim-bookworm

COPY src src
COPY Cargo.toml Cargo.toml

RUN cargo install --path .

CMD ["techjam-server"]
