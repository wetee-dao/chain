#!/usr/bin/env bash

# 来源于网络，用于获取当前shell文件的路径
SOURCE="$0"
while [ -h "$SOURCE"  ]; do # resolve $SOURCE until the file is no longer a symlink
    DIR="$( cd -P "$( dirname "$SOURCE"  )" && pwd  )"
    SOURCE="$(readlink "$SOURCE")"
    [[ $SOURCE != /*  ]] && SOURCE="$DIR/$SOURCE" # if $SOURCE was a relative symlink, we need to resolve it relative to the path where the symlink file was located
done
DIR="$( cd -P "$( dirname "$SOURCE"  )" && pwd  )"
cd "$DIR/../"
pwd

current=`date "+%Y-%m-%d-%H_%M"`
TAG="dev.$current"
ENV=`git symbolic-ref HEAD 2>/dev/null | cut -d"/" -f 3`

if [ $# -gt 0 ]; then
  TAG="$1.$current"
  if [ $# -gt 1 ]; then
    ENV=$2
  fi
fi

# 编译
cargo build --release -p parachain-node

docker build . -f pos_from_bin.Dockerfile -t "registry.cn-hangzhou.aliyuncs.com/wetee_dao/wetee-node:$TAG"

docker login --username=wetee registry.cn-hangzhou.aliyuncs.com

docker push "registry.cn-hangzhou.aliyuncs.com/wetee_dao/wetee-node:$TAG"
