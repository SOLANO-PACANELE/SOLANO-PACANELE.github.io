#!/bin/bash
cd "$(dirname "${BASH_SOURCE[0]}")"

time docker exec -it solana bash -c '
set -ex
cd /app
(
    mkdir -p .keys/build
    if ! [ -f .keys/pcn_authority.json ] ; then
        echo "GENERATE NEW KEYS"
        solana-keygen new --no-bip39-passphrase -o .keys/pcn_authority.json
        solana-keygen new --no-bip39-passphrase -o .keys/pcn_program.json
    else
        echo "FOUND EXISTING KEYS"
    fi
    solana-keygen pubkey .keys/pcn_authority.json > .keys/pcn_authority.pubkey
    solana-keygen pubkey .keys/pcn_program.json > .keys/pcn_program.pubkey

    solana program deploy -u devnet --keypair  .keys/pcn_authority.json --program-id .keys/pcn_program.json /docker/cargo_target/deploy/pacanele2.so
)
'
