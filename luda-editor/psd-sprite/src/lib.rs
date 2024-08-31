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
use std::collections::HashMap;
use std::sync::Arc;

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

pub type SpriteLoadedImages = HashMap<SpriteImageId, SpriteLoadedImage>;

pub struct SpriteLoadedImage {
    pub header: nimg::Header,
    pub skia_image: Arc<skia_safe::Image>,
}

///
/// example usage:
/// ```rust no_run no_compile
/// async fn test() -> anyhow::Result<()> {
///     let stream = namui::reqwest::get("http://httpbin.org/ip")
///         .await?
///         .bytes_stream();
///
///     decode_psd_sprite(stream.map(|x| match x {
///         Ok(bytes) => anyhow::Ok(bytes),
///         Err(err) => Err(anyhow::anyhow!(err)),
///     }))
///     .await;
///
///     todo!()
/// }
/// ```
///
pub async fn decode_psd_sprite(
    bytes_stream: impl futures_core::Stream<Item = anyhow::Result<bytes::Bytes>> + std::marker::Unpin,
) -> anyhow::Result<(PsdSprite, SpriteLoadedImages)> {
    use futures_util::{stream::*, AsyncReadExt};

    let mut zstd_decoder = async_compression::futures::bufread::ZstdDecoder::new(
        bytes_stream
            .map_err(|e| futures_util::io::Error::new(futures_util::io::ErrorKind::Other, e))
            .into_async_read(),
    );

    let psd_sprite_byte_length = u32::from_le_bytes({
        let mut bytes = [0u8; 4];
        zstd_decoder.read_exact(&mut bytes).await?;
        bytes
    }) as usize;

    let psd_sprite: PsdSprite = {
        let mut psd_sprite_bytes = vec![0u8; psd_sprite_byte_length];
        zstd_decoder.read_exact(&mut psd_sprite_bytes).await?;
        bincode::deserialize(&psd_sprite_bytes)?
    };

    let mut sprite_image_loading_task = HashMap::new();

    loop {
        let mut sprite_image_id_length_bytes = [0u8; 4];
        if 4 != zstd_decoder.read(&mut sprite_image_id_length_bytes).await? {
            break;
        }
        let sprite_image_id_length = u32::from_le_bytes(sprite_image_id_length_bytes) as usize;

        let sprite_image_id: SpriteImageId = {
            let mut bytes = vec![0u8; sprite_image_id_length];
            zstd_decoder.read_exact(&mut bytes).await?;
            bincode::deserialize(&bytes)?
        };

        let sprite_image_byte_length = u32::from_le_bytes({
            let mut bytes = [0u8; 4];
            zstd_decoder.read_exact(&mut bytes).await?;
            bytes
        }) as usize;

        let sprite_image_bytes = {
            let mut bytes = vec![0u8; sprite_image_byte_length];
            zstd_decoder.read_exact(&mut bytes).await?;
            bytes
        };

        let task = tokio::task::spawn_blocking(move || {
            let (header, pixels) = nimg::decode(&sprite_image_bytes)?;

            let width = header.width as i32;
            let height = header.height as i32;

            let image_info = match header.color_type {
                ColorType::Rgba8888 => skia_safe::ImageInfo::new_n32(
                    (width, height),
                    skia_safe::AlphaType::Unpremul,
                    None,
                ),
                ColorType::A8 => skia_safe::ImageInfo::new_a8((width, height)),
            };

            let row_bytes = match header.color_type {
                ColorType::Rgba8888 => width * 4,
                ColorType::A8 => width,
            } as usize;

            let skia_image = skia_safe::image::images::raster_from_data(
                &image_info,
                skia_safe::Data::new_copy(&pixels),
                row_bytes,
            )
            .ok_or(anyhow::anyhow!("Failed to create image from SpriteImage"))?;

            anyhow::Ok(SpriteLoadedImage {
                header,
                skia_image: skia_image.into(),
            })
        });

        sprite_image_loading_task.insert(sprite_image_id, task);
    }

    let sprite_loaded_images: HashMap<_, _> =
        futures_util::future::try_join_all(sprite_image_loading_task.into_iter().map(
            |(id, task)| async move {
                let image = task.await??;
                anyhow::Ok((id, image))
            },
        ))
        .await?
        .into_iter()
        .collect();

    Ok((psd_sprite, sprite_loaded_images))
}
