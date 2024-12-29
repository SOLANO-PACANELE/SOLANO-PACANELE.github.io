#!/bin/bash
set -ex
export MSYS_NO_PATHCONV=1
export MSYS2_ARG_CONV_EXCL="*"
export MSYS_NO_PATHCONV=1

docker rm -f solana || true
( cd docker && docker build . --tag solana )
docker run \
    -d --name solana -p 8899:8899  --ulimit nofile=1000000 \
    -e "RUST_LOG=warn"\
    -w "/app" \
    -v "$PWD:/app" \
    solana
sleep 1
until docker exec solana solana balance; do sleep 1; done
echo "OK"
sleep 3
echo "OK"
docker exec -it solana bash