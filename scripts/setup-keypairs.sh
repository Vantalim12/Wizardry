#!/bin/bash

# Setup script for generating keypairs

echo "WIZ-SACHI Distributor Keypair Setup"
echo "===================================="
echo ""

# Create keypairs directory
mkdir -p keypairs

# Generate collector keypair if it doesn't exist
if [ ! -f keypairs/collector.json ]; then
    echo "Generating collector keypair..."
    solana-keygen new --outfile keypairs/collector.json --no-bip39-passphrase
    echo "Collector keypair created: keypairs/collector.json"
    COLLECTOR_PUBKEY=$(solana-keygen pubkey keypairs/collector.json)
    echo "Collector public key: $COLLECTOR_PUBKEY"
    echo ""
else
    echo "Collector keypair already exists"
    echo ""
fi

# Generate distributor keypair if it doesn't exist
if [ ! -f keypairs/distributor.json ]; then
    echo "Generating distributor keypair..."
    solana-keygen new --outfile keypairs/distributor.json --no-bip39-passphrase
    echo "Distributor keypair created: keypairs/distributor.json"
    DISTRO_PUBKEY=$(solana-keygen pubkey keypairs/distributor.json)
    echo "Distributor public key: $DISTRO_PUBKEY"
    echo ""
else
    echo "Distributor keypair already exists"
    echo ""
fi

echo "Setup complete!"
echo ""
echo "IMPORTANT: Fund these keypairs with SOL for transactions."
echo "IMPORTANT: Store these keypairs securely and never commit them to git."

