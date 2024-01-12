FROM rust:buster as builder

RUN apt-get update && apt-get install time cmake clang libclang-dev llvm protobuf-compiler -y
RUN rustup toolchain install nightly-2023-08-22
RUN rustup target add wasm32-unknown-unknown --toolchain nightly-2023-08-22

WORKDIR /
COPY . .
RUN cargo build --locked --release


FROM ubuntu:22.04

## copy bin from builder
COPY  --from=builder  /target/release/wetee-node /usr/local/bin

## ubuntu update
RUN apt-get update


EXPOSE 30333 9933 9944 9615
CMD ["/usr/local/bin/wetee-node" "--dev"]