use dep::sha256;

fn main(
    // Private input: SHA-256 hash of image_bytes
    image_hash: [u8; 32],

    // Public inputs
    p_hash: [u8; 8],       // 64-bit perceptual hash
    platform_name: [u8; 32], // Fixed-size platform name
    timestamp: u64,        // Unix timestamp
    
    // Public output: hash of all inputs
    output_hash: pub [u8; 32]
) {
    // Combine inputs into a single array for hashing
    let mut input_bytes: [u8; 72] = [0; 72]; // 32 (image_hash) + 8 (p_hash) + 32 (platform_name) + 8 (timestamp)

    // Copy image_hash
    for i in 0..32 {
        input_bytes[i] = image_hash[i];
    }

    // Copy p_hash
    for i in 0..8 {
        input_bytes[32 + i] = p_hash[i];
    }

    // Copy platform_name
    for i in 0..32 {
        input_bytes[40 + i] = platform_name[i];
    }

    // Convert timestamp (u64) to 8 bytes (big-endian)
    let mut timestamp_bytes: [u8; 8] = [0; 8];
    for i in 0..8 {
        // Extract each byte: (timestamp >> (56 - 8*i)) & 0xFF
        let shift: u8 = (56 - (i as u64 * 8)) as u8; // Cast to u8
        timestamp_bytes[i] = ((timestamp >> shift) & 0xFF) as u8;
    }

    // Copy timestamp_bytes
    for i in 0..8 {
        input_bytes[64 + i] = timestamp_bytes[i];
    }

    // Compute SHA-256 of all inputs
    let computed_hash: [u8; 32] = sha256::digest(input_bytes);

    // Assert the computed hash matches the public output_hash
    for i in 0..32 {
        assert(computed_hash[i] == output_hash[i]);
    }
}