# WIZ-SACHI Token Distributor

Automated system that rewards $WIZ token holders with $SACHI tokens through continuous fee collection, token swapping, and proportional distribution.

## Overview

The WIZ-SACHI Token Distributor is an automated system that rewards $WIZ token holders with $SACHI tokens. The system continuously:

- Collects trading fees from the WIZ/SOL liquidity pool
- Converts SOL to SACHI using Jupiter aggregator for best rates
- Distributes SACHI proportionally to all WIZ holders based on their holdings

All operations run automatically every few minutes, ensuring consistent rewards for holders.

## Token Information

| Token                 | Contract Address                               | Type             |
| --------------------- | ---------------------------------------------- | ---------------- |
| $WIZ (Snapshot Token) | `<TO_BE_LAUNCHED_ON_PUMP.FUN>`                 | Token-2022 Token |
| $SACHI (Reward Token) | `7Y2TPeq3hqw21LRTCi4wBWoivDngCpNNJsN1hzhZpump` | Reward Token     |

## Features

- Fully Automated - Runs continuously with PM2 process management
- Proportional Rewards - Distribution based on WIZ holdings percentage
- Optimal Swaps - Jupiter aggregator finds best SOL→SACHI routes
- Pool Filtering - Automatically excludes AMM/DEX addresses
- Batch Processing - Efficient parallel transaction execution
- Secure - No private keys in code, environment-based configuration

## Prerequisites

- Rust 1.75+
- Node.js 20+ and npm
- Solana wallet keypairs (JSON format)
- RPC endpoint (Helius recommended)
- Make (for build automation)

## Installation

### 1. Clone the Repository

```bash
git clone <repository-url>
cd wizardry
```

### 2. Install Dependencies

```bash
# Install Rust dependencies and build binaries
make build-rust

# Install TypeScript dependencies
make build-ts
```

Or build everything at once:

```bash
make build
```

### 3. Configure Environment

```bash
cp .env.example .env
```

Edit `.env` with your settings:

```env
RPC_ENDPOINT=https://your-helius-endpoint.com
KEYPAIR_PATH=./keypairs/collector.json
KEYPAIR_DISTRO_PATH=./keypairs/distributor.json
WIZ_TOKEN_MINT=<UPDATE_AFTER_LAUNCH>
SACHI_TOKEN_MINT=7Y2TPeq3hqw21LRTCi4wBWoivDngCpNNJsN1hzhZpump
METEORA_POOL_ADDRESS=<TO_BE_CREATED>
```

### 4. Add Your Keypairs

Place your keypair files in the `keypairs/` directory:

```bash
mkdir -p keypairs
# Add collector.json and distributor.json
```

## Usage

### Development Mode

Run individual scripts:

```bash
# Collect pool fees
npm run dev:collect-fees

# Swap SOL to SACHI
npm run dev:swap

# Distribute tokens
npm run dev:distribute

# Create pool (one-time setup)
npm run dev:create-pool
```

### Production with PM2

Start all automated processes:

```bash
make deploy
```

Monitor processes:

```bash
make monitor
```

View logs:

```bash
make logs
```

Stop all processes:

```bash
make stop
```

## Project Structure

```
wizardry/
├── rust/
│   └── wiz-sachi-core/
│       ├── src/
│       │   ├── bin/          # Executable entry points
│       │   ├── collect_fees.rs
│       │   ├── swap_sol_to_sachi.rs
│       │   ├── distribute_tokens.rs
│       │   ├── create_pool.rs
│       │   └── utils.rs
│       └── Cargo.toml
├── typescript/
│   ├── src/                  # TypeScript wrappers
│   ├── dist/                  # Compiled output
│   └── package.json
├── ecosystem.config.js        # PM2 configuration
├── Dockerfile                 # Container definition
├── Makefile                   # Build automation
└── README.md
```

## How It Works

### 1. Fee Collection

Collects trading fees from WIZ/SOL pool on Meteora DLMM:

```bash
./rust/target/release/collect_fees
```

### 2. Automatic Swapping

Swaps accumulated SOL to SACHI tokens using Jupiter:

```bash
./rust/target/release/swap_sol_to_sachi
```

### 3. Reward Distribution

Takes snapshot of all WIZ holders and distributes SACHI:

```bash
./rust/target/release/distribute_tokens
```

## Distribution Logic

1. **Snapshot**: System scans all WIZ token accounts on-chain
2. **Filter**: Removes AMM pools, DEX programs, and blacklisted addresses
3. **Calculate**: Determines each holder's percentage of total supply
4. **Distribute**: Sends proportional SACHI rewards to all eligible holders

**Example**: If you hold 5% of circulating WIZ, you receive 5% of each SACHI distribution.

## PM2 Schedules

- Fee Collection: Every 2 minutes
- SOL→SACHI Swap: Every 2 minutes
- SACHI Distribution: Every 5 minutes

## Docker Deployment

Build and run with Docker:

```bash
docker build -t wiz-sachi-distributor .
docker run -d \
  --name distributor \
  --env-file .env \
  -v $(pwd)/keypairs:/app/keypairs \
  wiz-sachi-distributor
```

## Development

### Build Commands

```bash
make build          # Build everything
make build-rust     # Build Rust only
make build-ts       # Build TypeScript only
```

### Test Commands

```bash
make test           # Run all tests
make test-rust      # Test Rust
make test-ts        # Test TypeScript
```

### Clean Commands

```bash
make clean          # Remove all build artifacts
make clean-rust     # Clean Rust
make clean-ts       # Clean TypeScript
```

## Security Best Practices

- Never commit keypair files to version control
- Use environment variables for sensitive data
- Keep RPC endpoints private
- Regularly rotate wallet keys
- Monitor transaction logs for anomalies
- Use dedicated wallets for automated operations

## Environment Variables

| Variable             | Description                   | Required            |
| -------------------- | ----------------------------- | ------------------- |
| RPC_ENDPOINT         | Solana RPC URL                | Yes                 |
| KEYPAIR_PATH         | Path to fee collector keypair | Yes                 |
| KEYPAIR_DISTRO_PATH  | Path to distributor keypair   | Yes                 |
| WIZ_TOKEN_MINT       | WIZ token address             | Yes                 |
| SACHI_TOKEN_MINT     | SACHI token address           | Yes                 |
| METEORA_POOL_ADDRESS | Pool address                  | After pool creation |
| JUPITER_API_URL      | Jupiter API endpoint          | No (defaults)       |
| MIN_SOL_RESERVE      | SOL to reserve                | No (1.0 default)    |

## Troubleshooting

### Build Issues

```bash
# Clean and rebuild
make clean
make build
```

### PM2 Issues

```bash
# Restart all processes
pm2 restart all

# View error logs
pm2 logs --err
```

### RPC Issues

If experiencing RPC rate limits:

- Use Helius or a dedicated RPC endpoint
- Implement request queuing
- Add exponential backoff

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the LICENSE file for details.
