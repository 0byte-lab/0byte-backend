use dep::std::hash::poseidon;


/* -------------------------------------------------------------------------- */
/*                                IMAGE CIRCUIT                               */
/* -------------------------------------------------------------------------- */
fn main(
    // private inputs
    image_hash: [Field; 4],
    p_hash: Field,
    platform_name: [Field; 4],
    timestamp: Field,
    
    // Public outputs
    output_hash: pub [Field; 4],
    nullifier_hash: pub Field
) {
    // compute nullifier (a unique identifier that prevents proof reuse without revealing original data.)
    let nullifier = poseidon::bn254::hash_4([
        image_hash[0],
        image_hash[1],
        p_hash,
        platform_name[0]
    ]);
    
    // compute main hash
    let hash_part1 = poseidon::bn254::hash_4([image_hash[0], image_hash[1], image_hash[2], image_hash[3]]);
    let hash_part2 = poseidon::bn254::hash_4([p_hash, platform_name[0], platform_name[1], platform_name[2]]);
    let hash_part3 = poseidon::bn254::hash_2([platform_name[3], timestamp]);
    
    // verify outputs (constraints)
    assert(hash_part1 == output_hash[0]);
    assert(hash_part2 == output_hash[1]);
    assert(hash_part3 == output_hash[2]);
    assert(nullifier == nullifier_hash);
}