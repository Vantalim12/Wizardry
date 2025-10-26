#!/bin/bash

# Build script for WIZ-SACHI Distributor

set -e

echo "Building WIZ-SACHI Token Distributor..."
echo "========================================"
echo ""

# Build Rust components
echo "Step 1: Building Rust components..."
cd rust/wiz-sachi-core
cargo build --release
cd ../..
echo "✓ Rust build complete"
echo ""

# Build TypeScript components
echo "Step 2: Building TypeScript components..."
cd typescript
npm install
npm run build
cd ..
echo "✓ TypeScript build complete"
echo ""

echo "Build complete! Ready for deployment."
echo ""
echo "Next steps:"
echo "  1. Configure .env file"
echo "  2. Set up keypairs: bash scripts/setup-keypairs.sh"
echo "  3. Start with: pm2 start ecosystem.config.js"

