FROM ubuntu:22.04

## shell json解析包
RUN apt-get update
# RUN apt-get install -y jq
# RUN apt-get install -y curl
# RUN apt-get install -y httpie

# 复制
COPY /target/release/parachain-node /

EXPOSE 9933 9944 9615
VOLUME ["/chain-data"]

ENTRYPOINT ["/parachain-node"]