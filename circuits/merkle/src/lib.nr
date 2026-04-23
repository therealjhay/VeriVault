global TREE_DEPTH: u32 = 20;

fn poseidon2_hash(inputs: [Field; 2]) -> Field {
    std::hash::pedersen_hash(inputs)
}

/// Verify a Poseidon-based Merkle proof
pub fn verify_merkle_proof(
    leaf: Field,
    path_index: u32,
    siblings: [Field; TREE_DEPTH],
    root: Field
) -> bool {
    let mut computed_hash = leaf;
    for i in 0..TREE_DEPTH {
        let bit = (path_index >> i) & 1;
        if bit == 0 {
            computed_hash = poseidon2_hash([computed_hash, siblings[i]]);
        } else {
            computed_hash = poseidon2_hash([siblings[i], computed_hash]);
        }
    }
    computed_hash == root
}
