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

cargo build --features runtime-benchmarks --release -p wetee-node 

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

# # assets
# ./target/release/wetee-node benchmark pallet --chain dev \
#     --pallet wetee-assets --extrinsic '*' --steps 20 --repeat 10 \
#     --output packages/pallets/wetee-assets/src/weights.rs \
#     --template ./hacks/frame-weight-template.hbs

# # gov
# ./target/release/wetee-node benchmark pallet --chain dev \
#     --pallet wetee-gov --extrinsic '*' --steps 20 --repeat 10 \
#     --output packages/pallets/wetee-gov/src/weights.rs \
#     --template ./hacks/frame-weight-template.hbs

# # app
# ./target/release/wetee-node benchmark pallet --chain dev \
#     --pallet wetee-app --extrinsic '*' --steps 20 --repeat 10 \
#     --output packages/pallets/wetee-app/src/weights.rs \
#     --template ./hacks/frame-weight-template.hbs

# # gpu
# ./target/release/wetee-node benchmark pallet --chain dev \
#     --pallet wetee-gpu --extrinsic '*' --steps 20 --repeat 10 \
#     --output packages/pallets/wetee-gpu/src/weights.rs \
#     --template ./hacks/frame-weight-template.hbs

# # task
# ./target/release/wetee-node benchmark pallet --chain dev \
#     --pallet wetee-task --extrinsic '*' --steps 20 --repeat 10 \
#     --output packages/pallets/wetee-task/src/weights.rs \
#     --template ./hacks/frame-weight-template.hbs

# dsecret
./target/release/wetee-node benchmark pallet --chain dev \
    --pallet wetee-dsecret --extrinsic '*' --steps 20 --repeat 10 \
    --output packages/pallets/wetee-dsecret/src/weights.rs \
    --template ./hacks/frame-weight-template.hbs

# tee-bridge
./target/release/wetee-node benchmark pallet --chain dev \
    --pallet wetee-tee-bridge --extrinsic '*' --steps 20 --repeat 10 \
    --output packages/pallets/wetee-tee-bridge/src/weights.rs \
    --template ./hacks/frame-weight-template.hbs

# # tee-worker
# ./target/release/wetee-node benchmark pallet --chain dev \
#     --pallet wetee-worker --extrinsic '*' --steps 20 --repeat 10 \
#     --output packages/pallets/wetee-worker/src/weights.rs \
#     --template ./hacks/frame-weight-template.hbs