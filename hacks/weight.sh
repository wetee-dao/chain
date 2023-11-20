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

cargo build --release --features runtime-benchmarks

# # org
# ./target/release/wetee-node benchmark pallet --chain dev \
#     --pallet wetee-org --extrinsic '*' --steps 20 --repeat 10 \
#     --output packages/pallets/wetee-org/src/weights.rs \
#     --template ./hacks/frame-weight-template.hbs

# # sudo
# ./target/release/wetee-node benchmark pallet --chain dev \
#     --pallet wetee-sudo --extrinsic '*' --steps 20 --repeat 10 \
#     --output packages/pallets/wetee-sudo/src/weights.rs \
#     --template ./hacks/frame-weight-template.hbs

# # guild
# ./target/release/wetee-node benchmark pallet --chain dev \
#     --pallet wetee-guild --extrinsic '*' --steps 20 --repeat 10 \
#     --output packages/pallets/wetee-guild/src/weights.rs \
#     --template ./hacks/frame-weight-template.hbs

# # treasury
# ./target/release/wetee-node benchmark pallet --chain dev \
#     --pallet wetee-treasury --extrinsic '*' --steps 20 --repeat 10 \
#     --output packages/pallets/wetee-treasury/src/weights.rs \
#     --template ./hacks/frame-weight-template.hbs

# gov
./target/release/wetee-node benchmark pallet --chain dev \
    --pallet wetee-gov --extrinsic '*' --steps 20 --repeat 10 \
    --output packages/pallets/wetee-gov/src/weights.rs \
    --template ./hacks/frame-weight-template.hbs