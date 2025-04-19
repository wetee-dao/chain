# builder from hacks/builder.Dockerfile
FROM wetee/wetee-builder:2023-08-22 as builder

WORKDIR /
COPY . .
RUN cargo build --locked --release


# wetee-node
FROM ubuntu:22.04

## copy bin from builder
COPY  --from=builder  /target/release/parachain-node /usr/local/bin
COPY /wetee-paseo.json /

EXPOSE 9933 9944 9615
VOLUME ["/chain-data"]

CMD ["/bin/sh", "-c" ,"/parachain-node --collator --alice --rpc-external --rpc-methods=unsafe --unsafe-rpc-external --rpc-cors=all --chain /wetee-paseo.json --force-authoring --base-path  /chain-data -- --chain=paseo --sync fast-unsafe --blocks-pruning 256 --state-pruning 256"]

# /bin/sh -c '/parachain-node  --collator --name local --bootnodes /dns/paseo.asyou.me/tcp/30333/p2p/12D3KooWHNm9h3YHMyoP3CQDFXjjqbg2hbAkXZm6VmEYneADAuXw --rpc-external --rpc-methods=unsafe --unsafe-rpc-external --rpc-cors=all --chain /wetee-paseo.json --force-authoring --base-path  /chain-data -- --chain=paseo --sync fast-unsafe --blocks-pruning 256 --state-pruning 256'
# --bootnodes /dns/xiaobai.asyou.me/tcp/30333/p2p/12D3KooWJka5ZXvLAY5wpAYvbcojf7AuTC5t2Y3QuQGJ5jytf9qB
# /bin/sh -c '/parachain-node  --collator --name local --listen-addr /ip4/0.0.0.0/tcp/30333  --rpc-external --rpc-methods=unsafe --unsafe-rpc-external --rpc-cors=all --chain /wetee-paseo.json --force-authoring --base-path  /chain-data -- --chain=paseo --sync fast-unsafe --blocks-pruning 256 --state-pruning 256'