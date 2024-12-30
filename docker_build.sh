#!/bin/bash
set -ex
export MSYS_NO_PATHCONV=1
export MSYS2_ARG_CONV_EXCL="*"
export MSYS_NO_PATHCONV=1


( docker build . -f docker/Dockerfile --tag johnnysmitherson/pacanele:local_build && docker push johnnysmitherson/pacanele:local_build )