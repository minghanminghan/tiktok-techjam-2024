FROM rust:1.76

COPY src src
COPY Cargo.toml Cargo.toml
RUN apt-get update && apt-get install -y pkg-config

RUN cargo install --path .

CMD ["techjam-server"]
