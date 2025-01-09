#!/bin/bash
set -ex
cd "$(dirname "${BASH_SOURCE[0]}")"

export MSYS_NO_PATHCONV=1
export MSYS2_ARG_CONV_EXCL="*"
export MSYS_NO_PATHCONV=1


(
    cd docker
    docker build . -f Dockerfile.base --tag johnnysmitherson/pacanele:base 
    docker push johnnysmitherson/pacanele:base
)


# (
#     docker build . -f docker/Dockerfile.contract --tag johnnysmitherson/pacanele:contract 
#     docker push johnnysmitherson/pacanele:contract
# )


