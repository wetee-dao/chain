FROM ubuntu:22.04

## shell json解析包
RUN apt-get update
# RUN apt-get install -y jq
# RUN apt-get install -y curl
# RUN apt-get install -y httpie

# 复制
COPY /target/release/wetee-node /


EXPOSE 9933 9944 9615
VOLUME ["/chain-data"]

CMD ["/bin/sh", "-c" ,"/wetee-node --dev --base-path  /chain-data --rpc-external --rpc-methods=unsafe --unsafe-rpc-external --rpc-cors=all"]