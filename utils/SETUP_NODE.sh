#!/bin/bash

GETH_LOG_FILE="./logs/geth.log"
LIGHTHOUSE_LOG_FILE="./logs/lighthouse.log"

echo "Starting Ethereum Execution Node (geth)..."
geth --authrpc.addr localhost \
     --authrpc.port 8551 \
     --authrpc.vhosts localhost \
     --authrpc.jwtsecret ~/.ethereum/geth/jwtsecret >> "$GETH_LOG_FILE" 2>&1 &

echo "Starting Beacon Node (lighthouse)..."
lighthouse bn \
  --network mainnet \
  --execution-endpoint http://localhost:8551 \
  --execution-jwt ~/.ethereum/geth/jwtsecret \
  --disable-deposit-contract-sync \
  --http >> "$LIGHTHOUSE_LOG_FILE" 2>&1 &
#  --http \
#  --checkpoint-sync-url https://beaconstate.info >> "$LIGHTHOUSE_LOG_FILE" 2>&1 &
