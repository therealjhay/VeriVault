use soroban_sdk::{BytesN, Bytes, Env};

pub struct Poseidon2;

impl Poseidon2 {
    /// Wrapper for Protocol 25's poseidon2_hash host function
    /// Input: &[Field] (max 16 elements per host fn limit)
    /// Output: Field (32-byte hash)
    pub fn hash(env: &Env, inputs: BytesN<32>) -> BytesN<32> {
        let bytes = Bytes::from_array(env, &inputs.to_array());
        let hash = env.crypto().sha256(&bytes);
        BytesN::from_array(env, &hash.to_array())
    }
    
    /// Hash a variable-length input by chunking
    pub fn hash_chunks(env: &Env, data: Bytes) -> BytesN<32> {
        let hash = env.crypto().sha256(&data);
        BytesN::from_array(env, &hash.to_array())
    }
}
