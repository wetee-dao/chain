FROM rust:buster

RUN apt-get update && apt-get install time cmake clang libclang-dev llvm protobuf-compiler -y
RUN rustup toolchain install nightly-2023-08-22
RUN rustup target add wasm32-unknown-unknown --toolchain nightly-2023-08-22
