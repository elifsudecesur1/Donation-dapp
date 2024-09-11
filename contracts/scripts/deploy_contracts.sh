#!/bin/bash

echo "Deploying NFT contract..."
cargo wasm --manifest-path ./contracts/nft/Cargo.toml
wasmd tx wasm store ./target/wasm32-unknown-unknown/release/nft_contract.wasm --from wallet --gas auto --gas-adjustment 1.5

echo "Deploying Staking contract..."
cargo wasm --manifest-path ./contracts/staking/Cargo.toml
wasmd tx wasm store ./target/wasm32-unknown-unknown/release/staking_contract.wasm --from wallet --gas auto --gas-adjustment 1.5

echo "Deploying Governance contract..."
cargo wasm --manifest-path ./contracts/governanceCargo.toml
wasmd tx wasm store ./target/wasm32-unknown-unknown/release/governance_contract.wasm --from wallet --gas auto --gas-adjustment 1.5

echo "Deploying Milestone contract..."
cargo wasm --manifest-path ./contracts/milestone/Cargo.toml
wasmd tx wasm store ./target/wasm32-unknown-unknown/release/milestone_contract.wasm --from wallet --gas auto --gas-adjustment 1.5
