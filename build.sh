#!/bin/bash
set -ex


docker exec -it solana bash -c '
    set -ex
    cd /app/pacanele2 
     cargo build-sbf 
      cargo test-sbf 
       solana program deploy $CARGO_TARGET_DIR/deploy/pacanele2.so
     solana address -k $CARGO_TARGET_DIR/deploy/pacanele2-keypair.json > /app/pacanele2/program_address.txt
     set +x
     echo
     echo "PROGRAM ADDRESS = http://localhost:3000/address/$(cat /app/pacanele2/program_address.txt)"
     cargo run --example client
     '