FROM rust:1.76

ARG SERVER_ENV
ENV SERVER_ENV $SERVER_ENV
COPY src src
COPY Cargo.toml Cargo.toml

RUN cargo install --path .

CMD ["techjam-server"]
