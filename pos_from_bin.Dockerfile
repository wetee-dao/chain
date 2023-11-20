FROM ubuntu:22.10

## shell json解析包
RUN apt-get update
# RUN apt-get install -y jq
# RUN apt-get install -y curl
# RUN apt-get install -y httpie

# 复制
COPY /target/release/dao-entrance-node /usr/local/bin


EXPOSE 30333 9933 9944 9615
VOLUME ["/chain-data"]

CMD ["/usr/local/bin/dao-entrance-node","--dev","--unsafe-ws-external","--rpc-cors","all"]