#!/bin/bash
set -ex

cd /docker/validator && nohup bash -c "cd /docker/validator && (  while true; do solana-test-validator 2>&1 >/docker/validator/solana-test-validator-log-docker.txt || sleep 3; done ) &"

cd /solana_explorer && nohup bash -c "( cd /solana_explorer && while true; do pnpm start 2>&1 >/docker/validator/solana-explorer-log-docker.txt  || sleep 3; done )"

sleep 3
curl http://localhost:3000/ || true
curl http://localhost:3000/supply || true
curl http://localhost:3000/supply?cluster=custom || true
curl http://localhost:3000/tx/inspector?cluster=custom || true

sleep 12345678