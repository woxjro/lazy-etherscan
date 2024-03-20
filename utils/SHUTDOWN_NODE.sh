#!/bin/bash

echo "Shutting down Ethereum Execution Node (geth)..."
pkill geth

echo "Shutting down Beacon Node (lighthouse)..."
pkill lighthouse
