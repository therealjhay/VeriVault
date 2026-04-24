use soroban_sdk::{Env, BytesN};
use crate::storage::state::{ContractState, StorageKey};
use crate::contract::ContractError;

pub fn update_policy_root(
    env: &Env,
    new_policy_root: BytesN<32>,
) -> Result<(), ContractError> {
    let mut state = get_state(env)?;
    state.admin.require_auth();

    state.policy_root = new_policy_root;
    env.storage().instance().set(&StorageKey::State, &state);

    Ok(())
}

fn get_state(env: &Env) -> Result<ContractState, ContractError> {
    env.storage().instance()
        .get(&StorageKey::State)
        .ok_or(ContractError::NotInitialized)
}
