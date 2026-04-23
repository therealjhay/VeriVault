use soroban_sdk::{contracttype, BytesN, Bytes, Env, Symbol, Vec};
use crate::crypto::poseidon::Poseidon2;

pub const TREE_DEPTH: u32 = 20;

pub struct IncrementalMerkle {
    env: Env,
    storage_key: Symbol,
}

#[derive(Clone, Debug)]
#[contracttype]
pub struct MerkleProof {
    pub leaf_index: u32,
    pub siblings: Vec<BytesN<32>>,
}

impl IncrementalMerkle {
    pub fn new(env: &Env, storage_key: Symbol) -> Self {
        Self {
            env: env.clone(),
            storage_key,
        }
    }
    
    pub fn insert(&mut self, leaf: BytesN<32>) -> MerkleProof {
        let mut siblings = Vec::new(&self.env);
        for _ in 0..TREE_DEPTH {
            siblings.push_back(self.zero_hash());
        }
        
        MerkleProof {
            leaf_index: 0,
            siblings,
        }
    }
    
    pub fn verify(&self, leaf: BytesN<32>, proof: &MerkleProof) -> bool {
        let mut computed_hash = leaf;
        for i in 0..TREE_DEPTH {
            let sibling = proof.siblings.get(i).unwrap();
            let bit = (proof.leaf_index >> i) & 1;
            
            let mut preimage = [0u8; 64];
            if bit == 0 {
                preimage[..32].copy_from_slice(&computed_hash.to_array());
                preimage[32..].copy_from_slice(&sibling.to_array());
            } else {
                preimage[..32].copy_from_slice(&sibling.to_array());
                preimage[32..].copy_from_slice(&computed_hash.to_array());
            }
            let bytes = Bytes::from_array(&self.env, &preimage);
            computed_hash = Poseidon2::hash_chunks(&self.env, bytes);
        }
        computed_hash.to_array() == self.zero_hash().to_array()
    }
    
    pub fn root(&self) -> BytesN<32> {
        self.zero_hash()
    }
    
    fn zero_hash(&self) -> BytesN<32> {
        BytesN::from_array(&self.env, &[0u8; 32])
    }
}
