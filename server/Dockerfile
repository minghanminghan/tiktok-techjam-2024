FROM rust:1.76

COPY src src
COPY Cargo.toml Cargo.toml
COPY .env .env

RUN cargo install --path .

CMD ["techjam-server"]
