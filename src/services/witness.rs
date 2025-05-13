use acvm::FieldElement;
use sha2::{Sha256, Digest};
use chrono::Utc;

pub struct Witness {
    pub image_hash: [FieldElement; 4],
    pub p_hash: FieldElement,
    pub platform_name: [FieldElement; 4],
    pub timestamp: FieldElement,
}

pub fn generate_witness(
    image_bytes: &[u8],
    platform: &str,
) -> Witness {
    // SHA-256 of image
    let image_hash = Sha256::digest(image_bytes);
    let image_hash_fields = split_to_fields(&image_hash);
    
    // PHash
    let p_hash_bytes = super::phash::compute_phash(
        &crate::utils::image::load_image(image_bytes).unwrap()
    );
    let u64_val = u64::from_be_bytes(p_hash_bytes);

    // Convert BigUint to FieldElement if needed
    let p_hash = FieldElement::from(u64_val as u128);

    // Platform name (padded to 32 bytes)
    let mut platform_bytes = [0u8; 32];
    platform_bytes[..platform.len().min(32)].copy_from_slice(&platform.as_bytes()[..platform.len().min(32)]);
    let platform_fields = split_to_fields(&platform_bytes);
    
    Witness {
        image_hash: image_hash_fields,
        p_hash,
        platform_name: platform_fields,
        timestamp: FieldElement::from(Utc::now().timestamp() as i128),
    }
}

pub fn prepare_proof_inputs(witness: Witness) -> Vec<FieldElement> {
    let mut inputs = Vec::with_capacity(11); // 4 + 1 + 4 + 1 + 1
    
    // Private inputs
    inputs.extend_from_slice(&witness.image_hash);
    inputs.push(witness.p_hash);
    inputs.extend_from_slice(&witness.platform_name);
    inputs.push(witness.timestamp);
    
    // Public outputs (dummy values, will be computed during proof)
    inputs.extend_from_slice(&[FieldElement::zero(); 4]); // output_hash
    inputs.push(FieldElement::zero()); // nullifier
    
    inputs
}

fn split_to_fields(bytes: &[u8]) -> [FieldElement; 4] {
    let mut fields = [FieldElement::zero(); 4];
    for i in 0..4 {
        // grab bytes i*8 .. i*8+8
        let chunk: [u8; 8] = bytes[i*8..(i*8 + 8)]
            .try_into()
            .expect("slice with incorrect length");
        
        // big‐endian → u64 → u128 → FieldElement
        let val_u64 = u64::from_be_bytes(chunk);
        fields[i]    = FieldElement::from(val_u64 as u128);
    }
    fields
}
