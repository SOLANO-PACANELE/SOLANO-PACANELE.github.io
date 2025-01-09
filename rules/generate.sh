#!/bin/bash
set -ex
cd "$(dirname "${BASH_SOURCE[0]}")"

cargo run --bin generate --features generate
cargo fmt
