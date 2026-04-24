use soroban_sdk::{contracttype, Address, BytesN, Vec};

#[contracttype]
pub struct ContractState {
    pub admin: Address,
    pub asset: Address,
    pub merkle_root: BytesN<32>,
    pub policy_root: BytesN<32>,
    pub next_recovery_id: u64,
    pub guardians: Vec<Address>,
    pub recovery_timelock: u64,
}

#[contracttype]
pub enum StorageKey {
    State,
    Nullifier(BytesN<32>),
    CommitmentMeta(BytesN<32>),
    Recovery(u64),
    PolicyTimelock(BytesN<32>),
    MerkleSubtrees,
    NextLeafIndex,
}
