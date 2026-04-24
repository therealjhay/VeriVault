use soroban_sdk::{contract, contractimpl, contracterror, Address, BytesN, Bytes, Env, Vec};
use crate::storage::state::{ContractState, StorageKey};
use crate::core::{deposit, withdraw, policy, recovery};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum ContractError {
    AlreadyInitialized = 0,
    NotInitialized = 1,
    ProofVerificationFailed = 2,
    NullifierAlreadySpent = 3,
    Unauthorized = 4,
    InvalidAmount = 5,
    InvalidPublicInputs = 6,
}

#[contract]
pub struct VeriVault;

#[contractimpl]
impl VeriVault {
    pub fn initialize(
        env: Env,
        admin: Address,
        asset: Address,
        policy_root: BytesN<32>,
        guardian_set: Vec<Address>
    ) -> Result<(), ContractError> {
        if env.storage().instance().has(&StorageKey::State) {
            return Err(ContractError::AlreadyInitialized);
        }

        let state = ContractState {
            admin,
            asset,
            merkle_root: BytesN::from_array(&env, &[0u8; 32]),
            policy_root,
            next_recovery_id: 0,
            guardians: guardian_set,
            recovery_timelock: 86400, // 24 hours
        };

        env.storage().instance().set(&StorageKey::State, &state);
        env.storage().persistent().set(&StorageKey::NextLeafIndex, &0u32);
        
        Ok(())
    }

    pub fn deposit(
        env: Env,
        user: Address,
        amount: i128,
        commitment: BytesN<32>,
        encrypted_metadata: BytesN<64>
    ) -> Result<(), ContractError> {
        deposit::execute_deposit(&env, user, amount, commitment, encrypted_metadata)
    }

    pub fn withdraw(
        env: Env,
        proof: Bytes,
        public_inputs: Vec<BytesN<32>>,
        recipient: Address,
        amount: i128
    ) -> Result<(), ContractError> {
        withdraw::execute_withdraw(&env, proof, public_inputs, recipient, amount)
    }

    pub fn update_policy_root(
        env: Env,
        new_policy_root: BytesN<32>
    ) -> Result<(), ContractError> {
        policy::update_policy_root(&env, new_policy_root)
    }

    pub fn initiate_recovery(
        env: Env,
        guardian: Address
    ) -> Result<(), ContractError> {
        recovery::initiate_recovery(&env, guardian)
    }

    pub fn get_root(env: Env) -> Result<BytesN<32>, ContractError> {
        let state = Self::get_state(&env)?;
        Ok(state.merkle_root)
    }

    fn get_state(env: &Env) -> Result<ContractState, ContractError> {
        env.storage().instance()
            .get(&StorageKey::State)
            .ok_or(ContractError::NotInitialized)
    }
}
