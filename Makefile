.PHONY: all contracts circuits sdk-rust sdk-ts test clean help deploy-local

# Default target
all: contracts circuits sdk-rust sdk-ts

help:
	@echo "VeriVault Build System"
	@echo "-----------------------"
	@echo "make all           - Build everything (contracts, circuits, SDKs)"
	@echo "make contracts     - Build Soroban contracts to WASM"
	@echo "make circuits      - Compile and check Noir ZK circuits"
	@echo "make sdk-rust      - Build the Rust SDK"
	@echo "make sdk-ts        - Build the TypeScript SDK"
	@echo "make test          - Run all tests (Cargo, Nargo)"
	@echo "make clean         - Remove build artifacts"
	@echo "make deploy-local  - Run local deployment script (requires Stellar CLI/Docker)"

# --- Contracts ---
contracts:
	@echo "Building Soroban contracts..."
	cargo build --package verivault_pool --target wasm32-unknown-unknown --release

# --- ZK Circuits ---
circuits:
	@echo "Checking ZK circuits..."
	@export PATH="$$HOME/.nargo/bin:$$PATH" && nargo check --workspace

# --- SDKs ---
sdk-rust:
	@echo "Building Rust SDK..."
	cargo build --package rust_sdk

sdk-ts:
	@echo "Building TypeScript SDK..."
	cd sdk/ts && npm install && npm run build

# --- Testing ---
test:
	@echo "Running Cargo tests..."
	cargo test --workspace
	@echo "Running Nargo tests..."
	@export PATH="$$HOME/.nargo/bin:$$PATH" && nargo test --workspace

# --- Deployment ---
deploy-local:
	@echo "Running local deployment..."
	./scripts/deploy_local.sh

# --- Cleanup ---
clean:
	@echo "Cleaning Cargo..."
	cargo clean
	@echo "Cleaning Noir artifacts..."
	find circuits -name "target" -type d -exec rm -rf {} +
	find circuits -name "proofs" -type d -exec rm -rf {} +
	@echo "Cleaning TypeScript SDK..."
	rm -rf sdk/ts/dist sdk/ts/node_modules
