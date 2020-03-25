#!/usr/bin/env bash

echo "Updating relay list..."
set -e
cargo run -p mullvad-rpc --bin relay_list > dist-assets/relays.json
