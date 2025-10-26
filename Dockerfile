# Multi-stage build for Rust and Node.js

# Stage 1: Rust build
FROM rust:1.75 as rust-builder

WORKDIR /app

# Install system dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy Rust source
COPY rust/ ./rust/
RUN cd rust/wiz-sachi-core && cargo build --release

# Stage 2: Node.js runtime
FROM node:20-slim

WORKDIR /app

# Install PM2 globally
RUN npm install -g pm2

# Copy TypeScript source and install dependencies
COPY typescript/package*.json ./typescript/
RUN cd typescript && npm install

# Copy TypeScript source files
COPY typescript/src ./typescript/src/
COPY typescript/tsconfig.json ./typescript/
RUN cd typescript && npm run build

# Copy Rust binaries from previous stage
COPY --from=rust-builder /app/rust/target/release/collect_fees ./rust/target/release/
COPY --from=rust-builder /app/rust/target/release/swap_sol_to_sachi ./rust/target/release/
COPY --from=rust-builder /app/rust/target/release/distribute_tokens ./rust/target/release/
COPY --from=rust-builder /app/rust/target/release/create_pool ./rust/target/release/

# Copy PM2 configuration
COPY ecosystem.config.js ./
COPY Makefile ./

# Create directories for logs and keypairs
RUN mkdir -p logs keypairs

# Set environment variables
ENV NODE_ENV=production

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
  CMD pm2 ping || exit 1

# Start PM2
CMD ["pm2-runtime", "ecosystem.config.js"]

