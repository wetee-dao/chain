FROM docker.io/paritytech/ci-linux:production as builder
WORKDIR /
COPY . .
RUN cargo build --locked --release


FROM ubuntu:22.10

# 复制
COPY  --from=builder  /target/release/wetee-node /usr/local/bin

## shell json解析包
RUN apt-get update
# RUN apt-get install -y jq
# RUN apt-get install -y curl
# RUN apt-get install -y httpie


EXPOSE 30333 9933 9944 9615
VOLUME ["/chain-data"]

WORKDIR /chain-data

CMD ["/usr/local/bin/wetee-node"]