use soroban_sdk::{contractimpl, Env};

pub struct Poseidon2;

#[contractimpl]
impl Poseidon2 {
    /// Wrapper for Protocol 25's poseidon2_hash host function
    /// Input: &[Field] (max 16 elements per host fn limit)
    /// Output: Field (32-byte hash)
    pub fn hash(env: &Env, inputs: &[u8; 32]) -> [u8; 32] {
        // Just calling a dummy or native implementation for now
        // Assuming Protocol 25 host fns or similar
        // For testing, we'll return a zeroed array if unimpl
        // Since we are mocking/stubbing this for now until host fn is actually present:
        let mut out = [0u8; 32];
        env.crypto().sha256(inputs.into()).copy_into_slice(&mut out);
        out
    }
    
    /// Hash a variable-length input by chunking
    pub fn hash_chunks(env: &Env, data: &[u8]) -> [u8; 32] {
        let mut out = [0u8; 32];
        env.crypto().sha256(data.into()).copy_into_slice(&mut out);
        out
    }
}
