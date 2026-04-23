use ::merkle;
use ::policy;

global ATTR_COUNT: u32 = 5;
global TREE_DEPTH: u32 = 20;
global POLICY_EVAL_DEPTH: u32 = 4;

fn poseidon2_hash_3(inputs: [Field; 3]) -> Field {
    std::hash::pedersen_hash(inputs)
}

fn poseidon2_hash_1(inputs: [Field; 1]) -> Field {
    std::hash::pedersen_hash(inputs)
}

fn poseidon2_hash_5(inputs: [Field; 5]) -> Field {
    std::hash::pedersen_hash(inputs)
}

fn main(
    // Private inputs
    nullifier: Field,
    secret: Field,
    attrs: [Field; ATTR_COUNT],
    merkle_path: [Field; TREE_DEPTH],
    policy_witness: [Field; POLICY_EVAL_DEPTH],
    
    // Public inputs
    commitment: pub Field,
    merkle_root: pub Field,
    policy_root: pub Field,
    nullifier_hash: pub Field
) {
    // 1. Verify commitment binding
    let attrs_hash = poseidon2_hash_5(attrs);
    assert(poseidon2_hash_3([nullifier, secret, attrs_hash]) == commitment);
    
    // 2. Verify Merkle membership
    let leaf = poseidon2_hash_1([commitment]);
    let path_index: u32 = 0;
    assert(merkle::verify_merkle_proof(leaf, path_index, merkle_path, merkle_root));
    
    // 3. Policy evaluation
    assert(policy::evaluate_policy_zk(attrs, policy_root, policy_witness));
    
    // 4. Nullifier binding
    assert(nullifier_hash == poseidon2_hash_1([nullifier]));
}
