#!/bin/bash
set -ex

cd /docker/validator && nohup bash -c "(  while true; do solana-test-validator 2>&1 >/docker/validator/solana-test-validator-log-docker.txt || sleep 3; done ) &"



sleep 12345678