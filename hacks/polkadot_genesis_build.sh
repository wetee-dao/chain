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


### build genesis
./target/release/parachain-node export-genesis-state --chain wetee-polkadot ./meta/polkadot/genesis

### build wasm
./target/release/parachain-node export-genesis-wasm --chain wetee-polkadot ./meta/polkadot/genesis-wasm

### build spec
./target/release/parachain-node build-spec --disable-default-bootnode --chain wetee-polkadot > ./meta/polkadot/wetee-polkadot.json