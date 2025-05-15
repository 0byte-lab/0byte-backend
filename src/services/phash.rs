use image;
use imagehash;

/* -------------------------------------------------------------------------- */
/*                                COMPUTE PHASH                               */
/* -------------------------------------------------------------------------- */
pub fn compute_phash(image: &image::DynamicImage) -> [u8; 8] {
    // Convert to grayscale first
    let gray_image = image.to_luma8();
    let grey_dyn = image::DynamicImage::ImageLuma8(gray_image);
    
    // Create the perceptual hash hasher
    let hasher = imagehash::PerceptualHash::new();

    // Generate the perceptual hash as a Vec<u8>
    let phash = hasher.hash(&grey_dyn);

    let bytes = phash.to_bytes();
    bytes.try_into().expect("hash should be exactly 8 bytes")
}

