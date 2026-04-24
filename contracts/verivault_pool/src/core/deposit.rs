use soroban_sdk::{Env, Address, BytesN};
use soroban_sdk::token::TokenClient;
use crate::storage::state::{ContractState, StorageKey};
use crate::crypto::merkle::IncrementalMerkle;
use crate::contract::ContractError;

pub fn execute_deposit(
    env: &Env,
    user: Address,
    amount: i128,
    commitment: BytesN<32>,
    encrypted_metadata: BytesN<64>
) -> Result<(), ContractError> {
    user.require_auth();

    if amount <= 0 {
        return Err(ContractError::InvalidAmount); // Need to add to ContractError
    }

    let mut state = get_state(env)?;
    
    // Transfer tokens
    let token_client = TokenClient::new(env, &state.asset);
    token_client.transfer(&user, &env.current_contract_address(), &amount);

    // Update Merkle tree
    let mut tree = IncrementalMerkle::new(env);
    let index: u32 = env.storage().persistent().get(&StorageKey::NextLeafIndex).unwrap_or(0);
    
    tree.insert(commitment.clone(), index);
    
    state.merkle_root = tree.get_root();
    env.storage().instance().set(&StorageKey::State, &state);
    env.storage().persistent().set(&StorageKey::NextLeafIndex, &(index + 1));
    
    // Store metadata
    env.storage().persistent().set(&StorageKey::CommitmentMeta(commitment), &encrypted_metadata);

    Ok(())
}

fn get_state(env: &Env) -> Result<ContractState, ContractError> {
    env.storage().instance()
        .get(&StorageKey::State)
        .ok_or(ContractError::NotInitialized)
}
