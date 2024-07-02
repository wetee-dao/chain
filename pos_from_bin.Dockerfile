FROM ubuntu:22.04

## shell json解析包
RUN apt-get update
# RUN apt-get install -y jq
# RUN apt-get install -y curl
# RUN apt-get install -y httpie

# 复制
COPY /target/release/parachain-node /
COPY /meta/rococo/wetee-rococo.json /


EXPOSE 9933 9944 9615
VOLUME ["/chain-data"]

CMD ["/bin/sh", "-c" ,"/parachain-node --collator --alice --rpc-external --rpc-methods=unsafe --unsafe-rpc-external --rpc-cors=all --chain /wetee-rococo.json --force-authoring --base-path  /chain-data -- --chain=rococo --sync fast-unsafe --blocks-pruning 256 --state-pruning 256"]