mod encode;
mod layer_tree;
mod psd_sprite;
mod sk_position_image;

pub use bincode;
use namui_type::*;
pub use nimg;
pub use nimg::ColorType;
use psd::IntoRgba;
pub use psd_sprite::*;

/// # Byte Format
///
/// (zstd compressed.)
///
/// - serialized PsdSprite byte length: u32le
/// - serialized PsdSprite
/// - Sequence of:
///    - serialized sprite_image_id length: u32le
///    - serialized sprite_image_id
///    - sprite image byte length: u32le
///    - sprite image
///
pub fn encode_psd_sprite(
    psd_bytes: &[u8],
    filename: &str,
) -> anyhow::Result<(Vec<u8>, schema_0::PartsSprite)> {
    let psd = psd::Psd::from_bytes(psd_bytes)?;
    let wh = Wh::new(psd.psd_width(), psd.psd_height()).map(|x| (x as f32).px());
    let layer_trees = layer_tree::make_tree(&psd);
    let (psd_sprite, images) = layer_tree::into_psd_sprite(layer_trees, wh)?;
    let parts_sprite = psd_sprite.to_parts_sprite(filename.to_string());

    let serialized_psd_sprite = bincode::serialize(&psd_sprite)?;

    let mut tuples = images
        .into_iter()
        .map(|(sprite_image_id, bytes)| anyhow::Ok((bincode::serialize(&sprite_image_id)?, bytes)))
        .collect::<anyhow::Result<Vec<_>>>()?;

    // sort by image byte length, bigger first
    tuples.sort_by(|(_, image_bytes1), (_, image_bytes2)| {
        image_bytes2.len().cmp(&image_bytes1.len())
    });

    let mut output = Vec::with_capacity(
        4 + serialized_psd_sprite.len()
            + tuples
                .iter()
                .map(|(sprite_image_id, image)| 4 + sprite_image_id.len() + 4 + image.len())
                .sum::<usize>(),
    );

    output.extend_from_slice(&(serialized_psd_sprite.len() as u32).to_le_bytes());
    output.extend_from_slice(&serialized_psd_sprite);
    for (sprite_image_id, image_bytes) in tuples {
        output.extend_from_slice(&(sprite_image_id.len() as u32).to_le_bytes());
        output.extend_from_slice(&sprite_image_id);
        output.extend_from_slice(&(image_bytes.len() as u32).to_le_bytes());
        output.extend_from_slice(&image_bytes);
    }

    let zstd_compressed = zstd::encode_all(output.as_slice(), 9)?;

    Ok((zstd_compressed, parts_sprite))
}
