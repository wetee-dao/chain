# builder from hacks/builder.Dockerfile
FROM wetee/wetee-builder:2023-08-22 as builder

WORKDIR /
COPY . .
RUN cargo build --locked --release


# wetee-node
FROM ubuntu:22.04

## copy bin from builder
COPY  --from=builder  /target/release/parachain-node /usr/local/bin
COPY /wetee-rococo.json /

EXPOSE 9933 9944 9615
VOLUME ["/chain-data"]

CMD ["/bin/sh", "-c" ,"/parachain-node --collator --alice --rpc-external --rpc-methods=unsafe --unsafe-rpc-external --rpc-cors=all --chain /wetee-rococo.json --force-authoring --base-path  /chain-data -- --chain=rococo --sync fast-unsafe --blocks-pruning 256 --state-pruning 256"]