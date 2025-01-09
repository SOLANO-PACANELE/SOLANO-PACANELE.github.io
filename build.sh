#!/bin/bash
set -ex
cd "$(dirname "${BASH_SOURCE[0]}")"



# time docker exec -it solana bash -c '
# set -ex
# (        
#         cd /app/pacanele2_client
#         export CARGO_TARGET_DIR=/pacanele2_client_target
#         cargo build --all
# )
# '

time docker exec -it solana bash -c '
set -ex
(
        
        cd /app/pacanele2 
        export CARGO_TARGET_DIR=/docker/cargo_target
        cargo build-sbf 
        # cargo test-sbf 
        solana program deploy $CARGO_TARGET_DIR/deploy/pacanele2.so
        if [ a$(cat /app/pacanele2/program_address.txt || true) != a$(solana address -k $CARGO_TARGET_DIR/deploy/pacanele2-keypair.json) ]; then
                echo WARNING NEW SOLANA ADDRESZZZZZ !!!
                solana address -k $CARGO_TARGET_DIR/deploy/pacanele2-keypair.json > /app/pacanele2/program_address.txt
        fi
        set +x
        echo vvvvvvvvvvvvvvvvvvvvvv
        echo
        echo "PROGRAM ADDRESS = http://localhost:3000/address/$(cat /app/pacanele2/program_address.txt)"
        echo
        echo ^^^^^^^^^^^^^^^^^^^^^^
)
'

# time docker exec -it solana bash -c '
# set -ex
# (
#         cd /app/pacanele2_client
#         export CARGO_TARGET_DIR=/pacanele2_client_target
#         time cargo run
#         time cargo run
#         time cargo run
# )
# '

