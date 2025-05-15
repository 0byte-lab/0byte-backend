use std::error::Error;
use std::io::{Cursor};
use image::ImageFormat;
use png::{Decoder, Encoder};

pub fn embed_metadata(image_bytes: &[u8], txn_id: &str, platform_name: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    let format = image::guess_format(image_bytes)?;

    match format {
        ImageFormat::Png => embed_in_png(image_bytes, txn_id, platform_name),
        ImageFormat::Jpeg => embed_in_jpeg(image_bytes, txn_id, platform_name),
        _ => Err("Unsupported image format for metadata embedding".into()),
    }
}

fn embed_in_png(image_bytes: &[u8], txn_id: &str, platform_name: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    let decoder = Decoder::new(Cursor::new(image_bytes));
    let mut reader = decoder.read_info()?;
    let mut buf = vec![0; reader.output_buffer_size()];
    let info = reader.next_frame(&mut buf)?;
    let raw_data = &buf[..info.buffer_size()];

    let mut out = Vec::new();
    {
        let mut encoder = Encoder::new(&mut out, info.width, info.height);
        encoder.set_color(info.color_type);
        encoder.set_depth(info.bit_depth);
        let mut writer = encoder.write_header()?;

        // Combine metadata
        let keyword = b"0byte_proof";
        let text = format!("{}|{}", txn_id, platform_name).into_bytes();

        let mut chunk_data = Vec::with_capacity(keyword.len() + 1 + text.len());
        chunk_data.extend_from_slice(keyword);
        chunk_data.push(0); // Null separator
        chunk_data.extend_from_slice(&text);

        writer.write_chunk(png::chunk::tEXt, &chunk_data)?;
        writer.write_image_data(raw_data)?;
    }

    Ok(out)
}

fn embed_in_jpeg(image_bytes: &[u8], txn_id: &str, platform_name: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    const JPEG_SOI: &[u8] = &[0xFF, 0xD8];
    if !image_bytes.starts_with(JPEG_SOI) {
        return Err("Invalid JPEG file".into());
    }

    let mut out = Vec::with_capacity(image_bytes.len() + 128);
    out.extend_from_slice(&image_bytes[..2]); // SOI

    let comment = format!("0byte_txn:{}|{}", txn_id, platform_name);
    let data = comment.as_bytes();
    let length = (data.len() + 2) as u16;

    out.push(0xFF);
    out.push(0xFE); // COM marker
    out.extend_from_slice(&length.to_be_bytes());
    out.extend_from_slice(data);

    out.extend_from_slice(&image_bytes[2..]);
    Ok(out)
}
