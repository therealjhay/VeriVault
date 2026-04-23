# VeriVault: Compliance-Forward Privacy Protocol for Stellar

> A Rust-native, Soroban-based protocol enabling **private transactions with selective regulatory disclosure** — powered by Protocol 25's BN254 and Poseidon primitives.

---

## 🎯 Vision

Enable institutional and retail users to transact privately on Stellar **while provably satisfying compliance requirements** — without trusting a central operator, exposing sensitive data, or sacrificing auditability.

**Core innovation**: Zero-knowledge proofs that attest to *attributes* (e.g., "KYC verified", "not on OFAC list", "accredited investor") rather than revealing raw identity data.

---

## 🔐 Core Architecture

```
┌─────────────────┐     ┌─────────────────────┐     ┌─────────────────┐
│   User Client   │     │   VeriVault SDK     │     │ Soroban Contract│
│  (Rust/TS/WSM)  │────▶│  (Proof Generation) │────▶│  (Rust/Soroban) │
└─────────────────┘     └─────────────────────┘     └─────────────────┘
         │                         │                         │
         │ 1. deposit(asset, amt)  │                         │
         │────────────────────────▶│                         │
         │                         │ 2. createNote()         │
         │                         │    - nullifier          │
         │                         │    - secret             │
         │                         │    - compliance_attrs   │
         │                         │ 3. commitment =         │
         │                         │    Poseidon(nullifier∥secret∥attrs) │
         │                         │ 4. store commitment +   │
         │                         │    encrypted metadata   │
         │◀────────────────────────│                         │
         │ 5. note backup (client) │                         │
         │                         │                         │
         │ 6. withdraw(note)       │                         │
         │────────────────────────▶│                         │
         │                         │ 7. sync Merkle state    │
         │                         │ 8. generate ZK proof:   │
         │                         │    • I know preimage of │
         │                         │      a committed note   │
         │                         │    • attrs satisfy      │
         │                         │      policy rules       │
         │                         │    • nullifier unused   │
         │                         │ 9. submit proof +       │
         │                         │    withdrawal request   │
         │                         │────────────────────────▶│
         │                         │                         │ 10. verify proof via
         │                         │                         │     bn254_pairing()
         │                         │                         │ 11. mark nullifier spent
         │                         │                         │ 12. transfer assets
         │◀────────────────────────│◀────────────────────────│
```

---

## ⚙️ Cryptographic Design (Protocol 25 Native)

| Step | Operation | Primitive | Purpose |
|------|-----------|-----------|---------|
| **Commitment** | `Poseidon2(nullifier ∥ secret ∥ attrs_hash)` | `poseidon2_hash` | Bind note to user + compliance attributes |
| **Merkle Storage** | Incremental Merkle tree (depth=20) | Soroban storage | Efficient membership proofs |
| **ZK Proof** | Groth16 circuit proving: <br>• Knowledge of preimage <br>• attrs satisfy policy <br>• nullifier uniqueness | Noir + BN254 | Private, verifiable compliance |
| **On-Chain Verify** | `bn254_pairing(proof, vk)` | `bn254_pairing` | Constant-time verification |
| **Selective Disclosure** | Schnorr-like signature over decrypted attrs (optional) | BN254 scalar mul | Prove specific claims to auditors |

### Compliance Circuit Logic (Noir Pseudocode)

```noir
// circuits/compliance_withdraw/src/main.nr
fn main(
    nullifier: Field,
    secret: Field,
    attrs: [Field; ATTR_COUNT], // e.g., [kyc_level, geo_hash, accreditation_flag]
    policy_root: Field,         // Merkle root of allowed policy rules
    merkle_path: [Field; 20],
    commitment: Field
) {
    // 1. Reconstruct commitment
    let attrs_hash = poseidon2_hash(attrs);
    let computed_commitment = poseidon2_hash([nullifier, secret, attrs_hash]);
    assert(computed_commitment == commitment);

    // 2. Verify Merkle membership
    let leaf_hash = poseidon2_hash([commitment]);
    assert(check_merkle_proof(leaf_hash, merkle_path, policy_root));

    // 3. Policy enforcement: attrs must satisfy on-chain policy root
    let attrs_satisfy = evaluate_policy(attrs, policy_root);
    assert(attrs_satisfy == 1);

    // 4. Nullifier uniqueness is enforced on-chain (storage check)
}
```

---

## 📁 Repository Structure

```
VeriVault/
├── circuits/
│   ├── compliance_withdraw/   # Main withdrawal circuit with policy checks
│   │   └── src/main.nr
│   ├── attribute_proof/       # Standalone proof: "I satisfy policy X"
│   │   └── src/main.nr
│   ├── merkle/                # Reusable Merkle proof library
│   │   └── src/lib.nr
│   ├── lib/
│   │   ├── hash/              # Poseidon2 wrappers
│   │   ├── policy/            # Policy evaluation logic (ZK-friendly)
│   │   └── utils/             # Field arithmetic helpers
│   └── scripts/
│       ├── setup_keys.sh      # Trusted setup (testnet)
│       └── export_vk.rs       # Export verification key to contract
│
├── contracts/
│   └── verivault_pool/
│       ├── src/
│       │   ├── contract.rs          # Soroban entry point
│       │   ├── lib.rs
│       │   ├── core/
│       │   │   ├── deposit.rs       # Asset locking + commitment storage
│       │   │   ├── withdraw.rs      # Proof verification + fund release
│       │   │   ├── policy.rs        # On-chain policy registry (admin-managed)
│       │   │   ├── disclosure.rs    # Optional selective disclosure flow
│       │   │   └── recovery.rs      # Time-locked emergency recovery (multi-sig)
│       │   ├── crypto/
│       │   │   ├── merkle.rs        # Incremental Merkle (depth=20, Poseidon2)
│       │   │   ├── verifier.rs      # BN254 Groth16 verifier (host fn wrapper)
│       │   │   └── commitment.rs    # Commitment scheme utilities
│       │   ├── storage/
│       │   │   ├── state.rs         # Contract state (Merkle root, nullifiers)
│       │   │   ├── policy_reg.rs    # Policy registry storage
│       │   │   └── recovery.rs      # Recovery state (timelock, guardians)
│       │   ├── types/
│       │   │   ├── events.rs        # Structured events for indexers
│       │   │   ├── errors.rs        # Custom Soroban error codes
│       │   │   └── compliance.rs    # Attribute schema definitions
│       │   ├── utils/
│       │   │   ├── validation.rs    # Input sanitization
│       │   │   └── address.rs       # Stellar address decoding
│       │   ├── test.rs              # Unit tests (Soroban test framework)
│       │   └── integration_test.rs  # End-to-end flow tests
│       ├── Cargo.toml
│       └── ARCHITECTURE.md
│
├── sdk/
│   ├── src/
│   │   ├── client.rs          # Rust SDK (WASM-compatible)
│   │   ├── proof_gen/         # Noir prover integration (via nargo)
│   │   ├── merkle_sync.rs     # Client-side Merkle tree sync
│   │   ├── compliance/        # Attribute encoding + policy evaluation
│   │   └── recovery/          # Recovery flow helpers
│   ├── wasm/                  # Precompiled WASM for browser use
│   └── tests/
│
├── policies/                  # Machine-readable compliance policies
│   ├── example_kyc.yaml       # Sample policy: "kyc_level >= 2"
│   ├── compiler/              # Policy → Noir constraint compiler (Rust)
│   └── registry/              # On-chain policy root management
│
├── formal/                    # Formal verification specs (K Framework / Lean)
│   ├── contract_spec.lean
│   └── circuit_spec.lean
│
├── scripts/
│   ├── deploy_testnet.sh
│   ├── generate_trusted_setup.sh
│   └── audit_prep/            # Scripts for audit artifact generation
│
├── docs/
│   ├── threat-model.md        # Adversary model + mitigations
│   ├── compliance-guide.md    # How regulators can verify without doxxing
│   ├── recovery-protocol.md   # Emergency recovery flow
│   └── api-reference.md
│
├── Cargo.toml (workspace)
├── rust-toolchain.toml
├── noir.toml
└── README.md
```

---

## 🛡️ Security Model

### Defense-in-Depth Layers

1. **Cryptographic**
   - BN254 curves via Protocol 25 host functions (no external crypto)
   - Poseidon2 for ZK-friendly hashing (resistant to known attacks)
   - Nullifiers prevent double-spend; commitments hide linkage

2. **Contract-Level**
   - All state changes emit structured events for off-chain monitoring
   - Reentrancy guards via Soroban's atomic execution model
   - Nullifier storage uses `has()` + `put()` atomicity

3. **Policy Enforcement**
   - Policies are Merklized; updates require admin multi-sig + timelock
   - Circuit enforces policy *at proof time*; contract enforces policy root *at verify time*

4. **Recovery & Governance**
   - Time-locked emergency recovery (e.g., 72h delay) with 3/5 guardian multi-sig
   - Guardians cannot steal funds — only restore access to *verified* owners via ZK identity proof

5. **Formal Verification Hooks**
   - Critical functions annotated with `#[verified]` markers
   - Specs in Lean/K for model checking (work in progress)

### Known Threats & Mitigations

| Threat | Mitigation |
|--------|------------|
| Circuit bug leaks attributes | Formal spec + multiple independent audits + bug bounty |
| Policy registry compromise | Multi-sig admin + timelock + on-chain governance votes |
| Front-running withdrawal | Commit-reveal + nullifier binding prevents replay |
| Merkle tree manipulation | On-chain root validation + client-side sync verification |
| Trusted setup toxicity | Use testnet setup initially; plan for MPC ceremony for mainnet |

> ⚠️ **Status**: Unaudited. For research/testnet use only.

---

## 🚀 Getting Started

```bash
# 1. Install toolchain
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup target add wasm32-unknown-unknown
cargo install --locked stellar-cli

curl -L https://raw.githubusercontent.com/noir-lang/noirup/main/install | bash
noirup

# 2. Clone and build
git clone https://github.com/yourorg/VeriVault.git
cd VeriVault

# Build circuits
cd circuits/compliance_withdraw && nargo build && nargo test
cd ../attribute_proof && nargo build

# Build contracts
cd ../../contracts/verivault_pool
cargo build --target wasm32-unknown-unknown --release
cargo test

# 3. Run local Stellar standalone + deploy
stellar container start
stellar contract deploy \
  --wasm target/wasm32-unknown-unknown/release/verivault_pool.wasm \
  --source alice \
  --network standalone
```

---

## 🗺️ Roadmap

### Phase 1: Core Protocol (Q2 2026)
- [x] Commitment scheme + Merkle tree (Rust/Noir)
- [x] Basic withdrawal circuit (no policy)
- [x] Soroban contract skeleton with deposit/withdraw
- [ ] Testnet deployment + integration tests

### Phase 2: Compliance Engine (Q3 2026)
- [ ] Policy compiler: YAML → Noir constraints
- [ ] Selective disclosure module (Schnorr-style attribute proofs)
- [ ] Guardian recovery flow with timelock
- [ ] SDK: Rust + WASM bindings for client apps

### Phase 3: Production Hardening (Q4 2026)
- [ ] Formal verification of critical circuits
- [ ] Third-party security audit (ZK + Soroban)
- [ ] MPC trusted setup ceremony
- [ ] Mainnet deployment + governance handoff

### Phase 4: Ecosystem (2027)
- [ ] Freighter wallet plugin
- [ ] Compliance oracle integration (e.g., Chainlink KYC feeds)
- [ ] Cross-chain privacy bridges (via Stellar as hub)

---

## 💡 Why This Is Spectacular

1. **First-mover on Protocol 25**: Leverages new BN254/Poseidon host functions before competitors.
2. **Privacy + Compliance, not privacy vs compliance**: Solves the institutional adoption blocker.
3. **Rust-native end-to-end**: Contracts, SDK, and tooling in Rust — type-safe, memory-safe, auditable.
4. **Extensible policy framework**: Regulators, DAOs, or enterprises can define custom compliance rules without forking.
5. **Recovery without backdoors**: Emergency access via ZK identity proofs — no central key holder.

---

## 🤝 Contributing

We prioritize security and correctness over speed.

```markdown
1. Review [docs/threat-model.md] before coding
2. Claim an issue tagged `good-first-issue` or `audit-candidate`
3. All ZK circuit changes require:
   - Circuit test vectors
   - Constraint count benchmark
   - Peer review from crypto team
4. Contract changes require:
   - Soroban unit + integration tests
   - Gas usage analysis
   - Formal spec update (if critical path)
```

Funded via **Stellar Community Fund** + **Drips Wave**. Contributors earn USDC for merged PRs.

---

## 📜 License

MIT — see [LICENSE](LICENSE)

---