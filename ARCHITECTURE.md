# WIZ-SACHI Distributor Architecture

## Overview

The WIZ-SACHI Token Distributor is a hybrid Rust/TypeScript system that automates token rewards for $WIZ holders through three main processes:

1. **Fee Collection** - Collects trading fees from Meteora DLMM pool
2. **Token Swapping** - Converts SOL to SACHI via Jupiter aggregator
3. **Distribution** - Distributes SACHI proportionally to WIZ holders

## Architecture Components

### Rust Core (`rust/wiz-sachi-core/`)

Performance-critical operations implemented in Rust:

#### 1. Fee Collection (`collect_fees.rs`)

- Connects to Meteora DLMM pool
- Parses pool state to identify accumulated fees
- Builds and submits collect_fees transaction
- Handles transaction confirmation and error recovery

#### 2. Token Swapping (`swap_sol_to_sachi.rs`)

- Integrates with Jupiter Aggregator API
- Fetches optimal swap quotes
- Executes SOL → SACHI swaps with slippage protection
- Maintains SOL reserve for transaction fees

#### 3. Token Distribution (`distribute_tokens.rs`)

- Snapshot current WIZ holders via on-chain queries
- Filters out AMM pools and blacklisted addresses
- Calculates proportional SACHI distribution
- Batches transfers for efficiency
- Handles partial transaction failures gracefully

#### 4. Pool Creation (`create_pool.rs`)

- One-time setup for Meteora DLMM pool
- Configures bin parameters and fee tiers
- Sets up initial liquidity
- Returns pool address for configuration

### TypeScript Orchestration (`typescript/src/`)

Wrapper scripts that call Rust binaries:

- `collect-fees.ts` - Executes fee collection binary
- `swap-sol-to-sachi.ts` - Executes swap binary
- `distribute-tokens.ts` - Executes distribution binary
- `create-pool.ts` - Executes pool creation binary

Each wrapper handles:

- Environment variable loading
- Process spawning
- Error handling and logging
- Exit code management

## Data Flow

```
┌─────────────────┐
│  Meteora Pool   │
│  (WIZ/SOL)      │
└────────┬────────┘
         │ Trading fees accumulate
         ▼
┌─────────────────┐
│ Fee Collection  │ ← Every 2 min
│ (Rust Binary)   │
└────────┬────────┘
         │ SOL collected
         ▼
┌─────────────────┐
│   SOL Balance   │
│ (Collector)     │
└────────┬────────┘
         │ Minus 1 SOL reserve
         ▼
┌─────────────────┐
│ Swap to SACHI   │ ← Every 2 min
│  (Jupiter API)  │
└────────┬────────┘
         │ SACHI received
         ▼
┌─────────────────┐
│ SACHI Balance   │
│ (Distributor)   │
└────────┬────────┘
         │ Proportional distribution
         ▼
┌─────────────────┐
│   WIZ Holders   │ ← Every 5 min
│  (Filtered)      │
└─────────────────┘
```

## Process Scheduling

### PM2 Cron Configuration

```javascript
{
  collect_fees: {
    cron: '*/2 * * * *',  // Every 2 minutes
    script: 'typescript/dist/src/collect-fees.js'
  },
  swap_tokens: {
    cron: '*/2 * * * *',  // Every 2 minutes
    script: 'typescript/dist/src/swap-sol-to-sachi.js'
  },
  distribute: {
    cron: '*/5 * * * *',  // Every 5 minutes
    script: 'typescript/dist/src/distribute-tokens.js'
  }
}
```

## Distribution Algorithm

### Step 1: Snapshot

```rust
get_token_accounts(client, wiz_mint)
  → Returns: Vec<(Pubkey, u64)>  // (account, balance)
```

### Step 2: Filter

```rust
filtered = holders.filter(|&account| {
    !blacklist.contains(account) &&
    !is_amm_pool(account) &&
    !is_dex_program(account)
})
```

### Step 3: Calculate Proportions

```rust
total_supply = filtered.iter().map(|(_, amt)| amt).sum()
distributions = filtered.iter().map(|(account, wiz_amt)| {
    let percentage = wiz_amt / total_supply;
    let sachi_amt = sachi_balance * percentage;
    (account, sachi_amt)
})
```

### Step 4: Execute

```rust
// Batch transfers in parallel
batch_size = 10
for chunk in distributions.chunks(batch_size) {
    execute_transfers(chunk).await
}
```

## Security Architecture

### Keypair Management

- **Collector Keypair** (`KEYPAIR_PATH`)

  - Used for fee collection
  - Receives pool fees in SOL
  - Initiates SOL → SACHI swaps

- **Distributor Keypair** (`KEYPAIR_DISTRO_PATH`)
  - Used for token distribution
  - Holds SACHI to be distributed
  - Executes transfer transactions

### Environment Variables

All sensitive configuration via environment variables:

- RPC endpoints
- Keypair paths (never commited)
- Token addresses
- Pool addresses

### Transaction Safety

1. **Simulation** - All transactions simulated before execution
2. **Rate Limiting** - RPC requests throttled to avoid limits
3. **Error Recovery** - Failed transactions logged and retried
4. **Reserve Management** - SOL reserve prevents stuck state

## Deployment Models

### 1. PM2 (Recommended)

```bash
make build
make deploy
```

- Direct process management
- Easy monitoring with `pm2 monit`
- Automatic restarts on failure
- Memory limits enforced

### 2. Docker

```bash
docker-compose up -d
```

- Isolated environment
- Consistent across platforms
- Health checks included
- Volume mounts for keypairs

### 3. Manual

```bash
# Build binaries
cd rust/wiz-sachi-core && cargo build --release

# Run scripts
./rust/target/release/collect_fees
./rust/target/release/swap_sol_to_sachi
./rust/target/release/distribute_tokens
```

## Monitoring

### PM2 Logs

```bash
pm2 logs              # All logs
pm2 logs collect-fees # Specific process
pm2 monit            # Real-time monitoring
```

### Log Files

- `logs/collect-fees-out.log` - Fee collection stdout
- `logs/swap-tokens-out.log` - Swap operations stdout
- `logs/distribute-out.log` - Distribution stdout
- Error logs with `-error.log` suffix

### DTrace (Optional)

```bash
sudo dtrace -s .dtrace/monitor.d
```

Monitors:

- System calls per process
- RPC latency
- Memory usage patterns

## Error Handling

### Transaction Failures

- Logged with full context
- Retry with exponential backoff
- Alert if repeated failures

### RPC Failures

- Retry with different endpoints
- Fallback to public RPC
- Circuit breaker pattern

### Insufficient Funds

- Check balance before operations
- Reserve requirements enforced
- Graceful degradation

## Performance Optimization

### Batch Processing

Distributions batched in groups of 10:

- Reduces RPC load
- Faster overall execution
- Better error containment

### Parallel Execution

Multiple operations run concurrently:

- Fee collection and swapping overlap
- Independent of distribution timing

### Caching

Frequently accessed data cached:

- Token account snapshots
- Pool state
- RPC responses

## Future Enhancements

1. **Telemetry** - Metrics collection and dashboards
2. **Alerting** - Slack/Discord notifications
3. **Analytics** - Distribution history and trends
4. **Multi-chain** - Support for additional chains
5. **Governance** - On-chain configuration updates
