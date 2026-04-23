#!/bin/bash
# Start Stellar standalone in Docker
stellar container start || true

# Build and deploy contract
cd contracts/verivault_pool
stellar contract build
stellar contract deploy \
  --wasm target/wasm32-unknown-unknown/release/verivault_pool.wasm \
  --source testnet-admin \
  --network standalone \
  --rpc-url http://localhost:8000

echo "CONTRACT_ID=$(stellar contract id --wasm target/wasm32-unknown-unknown/release/verivault_pool.wasm --source testnet-admin)" > .env.local
