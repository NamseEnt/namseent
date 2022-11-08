use image::*;
use std::io::Cursor;

pub fn optimize_to_jpg(image_buffer: &[u8]) -> Result<Box<[u8]>, Box<dyn std::error::Error>> {
    let image = load_from_memory(&image_buffer)?;

    let mut output: Vec<u8> = Vec::new();
    image.write_to(&mut Cursor::new(&mut output), ImageOutputFormat::Jpeg(80))?;

    Ok(output.into_boxed_slice())
}
