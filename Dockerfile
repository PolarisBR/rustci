FROM rust:1.63.0

COPY app /source
WORKDIR /source

RUN cargo test
