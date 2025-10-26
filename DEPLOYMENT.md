# Deployment Guide

## Pre-Deployment Checklist

- [ ] Rust 1.75+ installed
- [ ] Node.js 20+ installed
- [ ] Solana CLI installed (for keypair generation)
- [ ] RPC endpoint configured
- [ ] WIZ token deployed on pump.fun
- [ ] SACHI token address confirmed
- [ ] Test wallets created and funded

## Quick Start

### 1. Build the Project

```bash
make build
```

This will:

- Compile all Rust binaries
- Install TypeScript dependencies
- Build TypeScript wrappers

### 2. Configure Environment

```bash
cp .env.example .env
```

Edit `.env` with your settings:

```env
RPC_ENDPOINT=https://mainnet.helius-rpc.com/?api-key=YOUR_KEY
KEYPAIR_PATH=./keypairs/collector.json
KEYPAIR_DISTRO_PATH=./keypairs/distributor.json
WIZ_TOKEN_MINT=<YOUR_WIZ_TOKEN_ADDRESS>
SACHI_TOKEN_MINT=7Y2TPeq3hqw21LRTCi4wBWoivDngCpNNJsN1hzhZpump
```

### 3. Generate Keypairs

```bash
bash scripts/setup-keypairs.sh
```

Or manually:

```bash
mkdir -p keypairs
solana-keygen new --outfile keypairs/collector.json
solana-keygen new --outfile keypairs/distributor.json
```

### 4. Fund Keypairs

Send SOL to both keypairs:

```bash
# Fund collector (for fees and swaps)
solana transfer COLLECTOR_ADDRESS 10 --allow-unfunded-recipient

# Fund distributor (for distributions)
solana transfer DISTRIBUTOR_ADDRESS 5 --allow-unfunded-recipient
```

### 5. Create Pool (One-Time)

After deploying WIZ token:

```bash
npm run create-pool
```

Update `.env` with the pool address.

### 6. Start Automated Processes

```bash
pm2 start ecosystem.config.js
pm2 save
```

### 7. Monitor

```bash
pm2 monit
```

## Deployment Methods

### Method 1: PM2 (Recommended for Linux/Mac)

Best for dedicated servers and VPS.

**Advantages:**

- Native process management
- Easy monitoring
- Automatic restarts
- Resource limits

**Commands:**

```bash
# Start
pm2 start ecosystem.config.js

# Stop
pm2 stop all

# Restart
pm2 restart all

# View logs
pm2 logs

# Monitor
pm2 monit
```

### Method 2: Docker

Best for cloud deployments and consistent environments.

**Build:**

```bash
docker build -t wiz-sachi-distributor .
```

**Run:**

```bash
docker run -d \
  --name distributor \
  --env-file .env \
  -v $(pwd)/keypairs:/app/keypairs:ro \
  -v $(pwd)/logs:/app/logs \
  wiz-sachi-distributor
```

**With Docker Compose:**

```bash
docker-compose up -d
docker-compose logs -f
```

### Method 3: Systemd (Linux)

Create service file:

```bash
sudo nano /etc/systemd/system/wiz-sachi.service
```

Add:

```ini
[Unit]
Description=WIZ-SACHI Token Distributor
After=network.target

[Service]
Type=simple
User=your-user
WorkingDirectory=/path/to/wizardry
ExecStart=/usr/bin/pm2 start ecosystem.config.js --no-daemon
Restart=always

[Install]
WantedBy=multi-user.target
```

Enable and start:

```bash
sudo systemctl enable wiz-sachi
sudo systemctl start wiz-sachi
sudo systemctl status wiz-sachi
```

## Production Deployment

### Server Requirements

- **CPU**: 2+ cores
- **RAM**: 4GB minimum, 8GB recommended
- **Storage**: 50GB for logs
- **Network**: Reliable high-speed connection
- **OS**: Ubuntu 22.04 LTS or similar

### Security Hardening

#### 1. Firewall

```bash
sudo ufw allow 22/tcp
sudo ufw enable
```

#### 2. User Permissions

```bash
# Create dedicated user
sudo useradd -m -s /bin/bash distributor
sudo chown -R distributor:distributor /path/to/wizardry
```

#### 3. Keypair Protection

```bash
# Secure keypairs directory
chmod 700 keypairs
chmod 600 keypairs/*.json
```

#### 4. Log Rotation

Create `/etc/logrotate.d/wiz-sachi`:

```
/path/to/wizardry/logs/*.log {
    daily
    rotate 30
    compress
    missingok
    notifempty
}
```

### Monitoring Setup

#### PM2 Monitoring

```bash
pm2 install pm2-logrotate
pm2 set pm2-logrotate:max_size 50M
pm2 set pm2-logrotate:retain 30
```

#### Health Checks

Monitor for:

- Success rate of operations
- RPC response times
- SOL balance levels
- Failed transactions

## Troubleshooting

### Build Fails

```bash
# Clean and rebuild
make clean
make build
```

### Runtime Errors

Check logs:

```bash
pm2 logs --err
tail -f logs/*error.log
```

### RPC Issues

- Verify endpoint in `.env`
- Check rate limits
- Try different RPC provider
- Add delay between requests

### Transaction Failures

Common causes:

1. Insufficient SOL for fees
2. RPC timeout
3. Network congestion
4. Invalid addresses

Solutions:

- Add more SOL to wallets
- Use dedicated RPC
- Adjust retry logic
- Verify token addresses

## Maintenance

### Daily

- Check logs for errors
- Verify processes running
- Monitor balances

### Weekly

- Review transaction history
- Clean old logs
- Update dependencies

### Monthly

- Rotate keypairs (if needed)
- Update RPC endpoints
- Review security settings

## Rollback Procedure

If issues detected:

1. Stop processes: `pm2 stop all`
2. Review logs to identify issue
3. Apply fixes or rollback code
4. Rebuild: `make build`
5. Restart: `pm2 restart all`

## Support

For issues:

1. Check logs first
2. Review error messages
3. Consult architecture docs
4. Open issue on GitHub
