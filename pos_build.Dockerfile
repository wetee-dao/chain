# builder from hacks/builder.Dockerfile
FROM wetee/wetee-builder:2023-08-22 as builder

WORKDIR /
COPY . .
RUN cargo build --locked --release


# wetee-node
FROM ubuntu:22.04

## copy bin from builder
COPY  --from=builder  /target/release/wetee-node /usr/local/bin

## ubuntu update
RUN apt-get update


EXPOSE 30333 9933 9944 9615

CMD ["/bin/sh", "-c" ,"/usr/local/bin/wetee-node --dev --rpc-external --rpc-methods=unsafe --unsafe-rpc-external --rpc-cors=all"]