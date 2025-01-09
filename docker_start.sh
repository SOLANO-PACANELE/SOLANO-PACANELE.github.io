#!/bin/bash
set -ex
cd "$(dirname "${BASH_SOURCE[0]}")"

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
    -v pacanele2_client_target:/pacanele2_client_target \
    johnnysmitherson/pacanele:base
    # solanalabs/solana:v1.18.26

sleep 2
until docker exec solana solana balance; do sleep 1; done
echo "SOLANA OK"
echo "visit http://localhost:3000"