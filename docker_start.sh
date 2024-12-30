#!/bin/bash
set -ex
export MSYS_NO_PATHCONV=1
export MSYS2_ARG_CONV_EXCL="*"
export MSYS_NO_PATHCONV=1


docker rm -f solana || true
docker run \
    -d --name solana --hostname solana -p 8899:8899 -p 3000:3000 --ulimit nofile=1000000 \
    -e "RUST_LOG=warn"\
    --memory 6G --memory-swap 6G \
    -w "/app" \
    -v "$PWD/:/app" \
    -v solana_cargo_target:/docker/cargo_target \
    johnnysmitherson/pacanele:local_build
    # solanalabs/solana:v1.18.26

sleep 2
until docker exec solana solana balance; do sleep 1; done
echo "SOLANA OK"
echo "visit http://localhost:3000"