.PHONY: build test deploy monitor clean

# Build configuration
RUST_DIR = rust
TS_DIR = typescript

# Build Rust binaries
build-rust:
	@echo "Building Rust components..."
	cd $(RUST_DIR)/wiz-sachi-core && cargo build --release
	@echo "Rust build complete"

# Build TypeScript
build-ts:
	@echo "Building TypeScript components..."
	cd $(TS_DIR) && npm install
	cd $(TS_DIR) && npm run build
	@echo "TypeScript build complete"

# Build everything
build: build-rust build-ts
	@echo "Build complete"

# Run tests
test-rust:
	@echo "Running Rust tests..."
	cd $(RUST_DIR)/wiz-sachi-core && cargo test
	@echo "Rust tests complete"

test-ts:
	@echo "Running TypeScript tests..."
	cd $(TS_DIR) && npm test || true
	@echo "TypeScript tests complete"

test: test-rust test-ts

# Deploy with PM2
deploy:
	@echo "Starting deployment..."
	npm install -g pm2 || true
	pm2 start ecosystem.config.js
	pm2 save
	@echo "Deployment complete. Use 'pm2 monit' to monitor."

# Start monitoring
monitor:
	pm2 monit

# View logs
logs:
	pm2 logs

# Stop all processes
stop:
	pm2 stop all

# Clean build artifacts
clean-rust:
	cd $(RUST_DIR)/wiz-sachi-core && cargo clean

clean-ts:
	cd $(TS_DIR) && rm -rf dist node_modules

clean: clean-rust clean-ts
	@echo "Clean complete"

# Help
help:
	@echo "Available targets:"
	@echo "  build      - Build Rust and TypeScript components"
	@echo "  build-rust - Build Rust components only"
	@echo "  build-ts   - Build TypeScript components only"
	@echo "  test       - Run all tests"
	@echo "  deploy     - Deploy with PM2"
	@echo "  monitor    - Start PM2 monitoring"
	@echo "  logs       - View PM2 logs"
	@echo "  stop       - Stop all PM2 processes"
	@echo "  clean      - Remove build artifacts"
	@echo "  help       - Show this help message"

