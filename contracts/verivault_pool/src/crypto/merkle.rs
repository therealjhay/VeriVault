use soroban_sdk::{contracttype, BytesN, Bytes, Env, Symbol, Vec};
use crate::crypto::poseidon::Poseidon2;

pub const TREE_DEPTH: u32 = 20;

#[derive(Clone, Debug)]
#[contracttype]
pub struct MerkleProof {
    pub leaf_index: u32,
    pub siblings: Vec<BytesN<32>>,
}

pub struct IncrementalMerkle {
    env: Env,
}

impl IncrementalMerkle {
    pub fn new(env: &Env) -> Self {
        Self { env: env.clone() }
    }

    /// Returns the current root of the tree.
    /// In a real implementation, we would store the 'filled_subtrees' in persistent storage.
    pub fn get_root(&self) -> BytesN<32> {
        let subtrees = self.get_filled_subtrees();
        let mut current_hash = subtrees.get(0).unwrap_or(self.zero_hash(0));
        
        // This is a simplified version; a true incremental tree maintains 
        // the root by hashing the filled subtrees with zeros for the remaining path.
        for i in 0..TREE_DEPTH {
            current_hash = self.hash_pair(current_hash, self.zero_hash(i));
        }
        current_hash
    }

    pub fn insert(&mut self, leaf: BytesN<32>, index: u32) -> MerkleProof {
        let mut subtrees = self.get_filled_subtrees();
        let mut siblings = Vec::new(&self.env);
        let mut current_hash = leaf;

        for i in 0..TREE_DEPTH {
            if (index >> i) & 1 == 1 {
                let left = subtrees.get(i).unwrap();
                current_hash = self.hash_pair(left, current_hash);
            } else {
                subtrees.set(i, current_hash.clone());
                current_hash = self.hash_pair(current_hash, self.zero_hash(i));
            }
            siblings.push_back(self.zero_hash(i));
        }

        self.set_filled_subtrees(subtrees);

        MerkleProof {
            leaf_index: index,
            siblings,
        }
    }

    fn hash_pair(&self, left: BytesN<32>, right: BytesN<32>) -> BytesN<32> {
        let mut preimage = [0u8; 64];
        preimage[..32].copy_from_slice(&left.to_array());
        preimage[32..].copy_from_slice(&right.to_array());
        Poseidon2::hash_chunks(&self.env, Bytes::from_array(&self.env, &preimage))
    }

    fn zero_hash(&self, _level: u32) -> BytesN<32> {
        // In production, these should be precomputed
        BytesN::from_array(&self.env, &[0u8; 32])
    }

    fn get_filled_subtrees(&self) -> Vec<BytesN<32>> {
        self.env.storage().persistent()
            .get(&Symbol::new(&self.env, "subtrees"))
            .unwrap_or(Vec::new(&self.env))
    }

    fn set_filled_subtrees(&self, subtrees: Vec<BytesN<32>>) {
        self.env.storage().persistent().set(&Symbol::new(&self.env, "subtrees"), &subtrees);
    }
}
