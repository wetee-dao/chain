FROM ubuntu:24.04

## shell json解析包
RUN apt-get update && apt-get install -y ca-certificates

COPY /target/release/wetee-node /

EXPOSE 9933 9944 9615
VOLUME ["/chain-data"]

ENTRYPOINT ["/wetee-node","--dev","--rpc-external","--rpc-methods=unsafe","--unsafe-rpc-external","--rpc-cors=all","--base-path","/chain-data"]