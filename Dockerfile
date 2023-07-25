FROM rust:latest as builder

WORKDIR /usr/src/spip
COPY . .

RUN cargo build --release

