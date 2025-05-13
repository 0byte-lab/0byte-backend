use image::DynamicImage;

pub fn load_image(image_bytes: &[u8]) -> Result<DynamicImage, image::ImageError> {
    let image = image::load_from_memory(image_bytes)?;
    Ok(image)
}