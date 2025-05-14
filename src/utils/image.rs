use image::DynamicImage;
use actix_multipart::Multipart;
use futures_util::stream::StreamExt as _;
use crate::models::metadata::Metadata;

pub fn load_image(image_bytes: &[u8]) -> Result<DynamicImage, image::ImageError> {
    let image = image::load_from_memory(image_bytes)?;
    Ok(image)
}

pub async fn read_image_bytes(mut payload: Multipart) -> Result<(Vec<u8>, Metadata), Box<dyn std::error::Error>> {
    let mut image_bytes = vec![];
    let metadata = Metadata {
        model: "dall-e".into(),
        input_token: "prompt".into(),
        output_token: "generated image".into(),
    };
    
    while let Some(item) = payload.next().await {
        let mut field = item?;
        while let Some(chunk) = field.next().await {
            let data = chunk?;
            image_bytes.extend_from_slice(&data);
        }
    }
    Ok((image_bytes, metadata))
}