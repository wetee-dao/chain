#!/usr/bin/env bash

# 用于获取当前shell文件的路径
SOURCE="$0"
while [ -h "$SOURCE"  ]; do # resolve $SOURCE until the file is no longer a symlink
    DIR="$( cd -P "$( dirname "$SOURCE"  )" && pwd  )"
    SOURCE="$(readlink "$SOURCE")"
    [[ $SOURCE != /*  ]] && SOURCE="$DIR/$SOURCE" # if $SOURCE was a relative symlink, we need to resolve it relative to the path where the symlink file was located
done
DIR="$( cd -P "$( dirname "$SOURCE"  )" && pwd  )"
cd "$DIR/../"

./target/release/wetee-node build-spec --disable-default-bootnode --chain local > ./meta/local.json
./target/release/wetee-node --base-path ./db --chain=local --force-authoring --validator --name local --unsafe-rpc-external --rpc-cors all