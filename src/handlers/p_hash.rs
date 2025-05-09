use image::imageops::FilterType;
use rustdct::DctPlanner;
use log::{error, trace};

pub fn compute_phash(image_bytes: &[u8]) -> Result<String, Box<dyn std::error::Error>> {
    trace!("Starting pHash computation");

    // Load and decode the image
    let img = image::load_from_memory(image_bytes)
        .map_err(|e| {
            error!("Failed to load image: {}", e);
            format!("Failed to load image: {}", e)
        })?;

    // Convert to grayscale and resize to 32x32 for DCT
    let gray_img = img
        .grayscale()
        .resize_exact(32, 32, FilterType::Lanczos3)
        .to_luma8();

    // Extract pixel data
    let pixels: Vec<f32> = gray_img
        .pixels()
        .map(|p| p[0] as f32)
        .collect();

    // Perform 2D DCT (Discrete Cosine Transform)
    let mut planner = DctPlanner::new();
    let dct = planner.plan_dct2(32);
    let mut buffer = pixels;
    
    // Apply DCT row-wise
    for row in buffer.chunks_exact_mut(32) {
        dct.process_dct2(row);
    }

    // Apply DCT column-wise
    let mut transposed = vec![0.0f32; 32 * 32];
    for i in 0..32 {
        for j in 0..32 {
            transposed[j * 32 + i] = buffer[i * 32 + j];
        }
    }
    for col in transposed.chunks_exact_mut(32) {
        dct.process_dct2(col);
    }

    // Extract the top-left 8x8 region of DCT coefficients
    let mut coefficients = vec![];
    for i in 0..8 {
        for j in 0..8 {
            coefficients.push(transposed[i * 32 + j]);
        }
    }

    // Compute median of coefficients
    let mut sorted_coeffs = coefficients.clone();
    sorted_coeffs.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let median = if sorted_coeffs.len() % 2 == 0 {
        (sorted_coeffs[sorted_coeffs.len() / 2 - 1] + sorted_coeffs[sorted_coeffs.len() / 2]) / 2.0
    } else {
        sorted_coeffs[sorted_coeffs.len() / 2]
    };

    // Generate 64-bit hash by comparing coefficients to median
    let mut hash_bits = vec![];
    for coeff in coefficients {
        hash_bits.push(if coeff > median { 1 } else { 0 });
    }

    // Convert bits to hex string
    let mut hash_bytes = [0u8; 8];
    for (i, bit) in hash_bits.iter().enumerate() {
        hash_bytes[i / 8] |= (bit << (7 - (i % 8))) as u8;
    }
    let hash = hex::encode(hash_bytes);

    trace!("pHash computation completed successfully");
    Ok(hash)
}