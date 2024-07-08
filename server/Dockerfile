FROM rust:1.76

ENV SQLX_OFFLINE true
COPY src src
COPY Cargo.toml Cargo.toml

RUN cargo install --path .

CMD ["techjam-server"]
