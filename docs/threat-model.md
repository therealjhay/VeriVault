## Critical Invariants
- [x] Nullifier uniqueness: storage check + circuit binding
- [x] Commitment binding: Poseidon2 preimage resistance
- [x] Policy enforcement: circuit + contract root match
- [x] Recovery safety: timelock + multi-sig + no fund movement

## Adversary Capabilities
- Can observe all on-chain data (commitments, roots, events)
- Can submit arbitrary proofs/transactions
- Cannot break BN254 DLOG or Poseidon2 preimage resistance
- Cannot forge guardian signatures (ECDSA)

## Mitigations Implemented
1. All proofs verified on-chain before state change
2. Nullifiers checked before AND after proof verification (TOCTOU safe)
3. Policy updates require timelock + event emission
4. Recovery requires 3/5 guardian signatures + user ZK identity proof
