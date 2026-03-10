FROM ubuntu:24.04

## shell json解析包
RUN apt-get update && apt-get install -y ca-certificates

COPY /target/release/revive-dev-node /

EXPOSE 9933 9944 9615
VOLUME ["/chain-data"]

ENTRYPOINT ["/revive-dev-node","--dev","--rpc-external","--rpc-methods=unsafe","--unsafe-rpc-external","--rpc-cors=all","--base-path","/chain-data"]