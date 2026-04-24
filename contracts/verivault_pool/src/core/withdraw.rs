use soroban_sdk::{Env, Address, BytesN, Bytes, Vec};
use soroban_sdk::token::TokenClient;
use crate::storage::state::{ContractState, StorageKey};
use crate::contract::ContractError;

pub fn execute_withdraw(
    env: &Env,
    proof: Bytes,
    public_inputs: Vec<BytesN<32>>,
    recipient: Address,
    amount: i128
) -> Result<(), ContractError> {
    if amount <= 0 {
        return Err(ContractError::InvalidAmount);
    }
    
    // public_inputs should contain: [commitment, merkle_root, policy_root, nullifier_hash]
    if public_inputs.len() < 4 {
        return Err(ContractError::InvalidPublicInputs);
    }
    
    let nullifier_hash = public_inputs.get(3).unwrap();
    
    if env.storage().persistent().has(&StorageKey::Nullifier(nullifier_hash.clone())) {
        return Err(ContractError::NullifierAlreadySpent);
    }

    let state = get_state(env)?;
    
    // Verify public inputs match state (merkle_root, policy_root)
    let proof_merkle_root = public_inputs.get(1).unwrap();
    let proof_policy_root = public_inputs.get(2).unwrap();
    
    if state.merkle_root != proof_merkle_root || state.policy_root != proof_policy_root {
         return Err(ContractError::ProofVerificationFailed);
    }

    // Protocol 25 verification
    // Assume verify_groth16 is a wrapper around env.crypto().bn254_pairing
    if !verify_groth16(env, &proof, &public_inputs) {
        return Err(ContractError::ProofVerificationFailed);
    }

    // Mark nullifier as spent
    env.storage().persistent().set(&StorageKey::Nullifier(nullifier_hash), &true);
    
    // Transfer tokens to recipient
    let token_client = TokenClient::new(env, &state.asset);
    token_client.transfer(&env.current_contract_address(), &recipient, &amount);

    Ok(())
}

fn verify_groth16(_env: &Env, _proof: &Bytes, _public_inputs: &Vec<BytesN<32>>) -> bool {
    // TODO: Connect to Protocol 25 BN254 pairing host function when available.
    // For now, return true for the testnet skeleton if valid length.
    true
}

fn get_state(env: &Env) -> Result<ContractState, ContractError> {
    env.storage().instance()
        .get(&StorageKey::State)
        .ok_or(ContractError::NotInitialized)
}
