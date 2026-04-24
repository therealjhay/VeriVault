use soroban_sdk::{Env, Address};
use crate::storage::state::{ContractState, StorageKey};
use crate::contract::ContractError;

pub fn initiate_recovery(
    _env: &Env,
    _guardian: Address
) -> Result<(), ContractError> {
    // TODO: implement 3/5 multi-sig time-locked recovery logic
    Ok(())
}

fn _get_state(env: &Env) -> Result<ContractState, ContractError> {
    env.storage().instance()
        .get(&StorageKey::State)
        .ok_or(ContractError::NotInitialized)
}
