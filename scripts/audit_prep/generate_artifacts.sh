#!/bin/bash
set -e

echo "Generating audit bundle..."

for circuit in circuits/*/; do
  if [ -d "$circuit" ] && [ -f "$circuit/Nargo.toml" ]; then
    cd "$circuit"
    echo "Generating constraints for $circuit"
    nargo info --json > ../../audit_artifacts/$(basename "$circuit")_constraints.json || true
    cd ../..
  fi
done

cargo doc --no-deps --document-private-items --manifest-path contracts/verivault_pool/Cargo.toml || true
cp -r target/doc/* audit_artifacts/contract_docs/ || true

echo "Audit bundle ready in audit_artifacts/"
