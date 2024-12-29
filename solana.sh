#!/bin/bash
set -ex
export MSYS_NO_PATHCONV=1
export MSYS2_ARG_CONV_EXCL="*"
export MSYS_NO_PATHCONV=1

docker rm -f solana || true
docker run \
    -d --name solana -p 8899:8899 --ulimit nofile=1000000 \
    -e "RUST_LOG=warn"\
    -w "/app" \
    -v "$PWD:/app" \
    solanalabs/solana:v1.18.26
docker logs -f solana