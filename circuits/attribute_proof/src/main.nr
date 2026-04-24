use ::policy;

global ATTR_COUNT: u32 = 5;
global POLICY_EVAL_DEPTH: u32 = 4;

fn poseidon2_hash_5(inputs: [Field; 5]) -> Field {
    std::hash::pedersen_hash(inputs)
}

fn poseidon2_hash_3(inputs: [Field; 3]) -> Field {
    std::hash::pedersen_hash(inputs)
}

fn main(
    // Private inputs
    nullifier: Field,
    secret: Field,
    attrs: [Field; ATTR_COUNT],
    policy_witness: [Field; POLICY_EVAL_DEPTH],
    
    // Public inputs
    commitment: pub Field,
    policy_root: pub Field
) {
    // 1. Verify commitment binding to prove ownership of attributes
    let attrs_hash = poseidon2_hash_5(attrs);
    assert(poseidon2_hash_3([nullifier, secret, attrs_hash]) == commitment);
    
    // 2. Policy evaluation (Selective disclosure)
    assert(policy::evaluate_policy_zk(attrs, policy_root, policy_witness));
}
