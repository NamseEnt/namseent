mod blender;
mod render_psd_sprite;

use namui::*;
use psd_sprite::{PsdSprite, SpriteImageId};
pub use render_psd_sprite::*;
use std::collections::HashMap;

pub type SpriteLoadedImages = HashMap<SpriteImageId, namui::Image>;

///
/// example usage:
/// ```rust no_run
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
        psd_sprite::bincode::deserialize(&psd_sprite_bytes)?
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
            psd_sprite::bincode::deserialize(&bytes)?
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

        let task = namui::tokio::task::spawn_blocking(move || {
            let (header, pixels) = psd_sprite::nimg::decode(&sprite_image_bytes)?;

            let width = header.width as i32;
            let height = header.height as i32;

            let image_info = match header.color_type {
                psd_sprite::ColorType::Rgba8888 => skia_safe::ImageInfo::new_n32(
                    (width, height),
                    skia_safe::AlphaType::Unpremul,
                    None,
                ),
                psd_sprite::ColorType::A8 => skia_safe::ImageInfo::new_a8((width, height)),
            };

            let row_bytes = match header.color_type {
                psd_sprite::ColorType::Rgba8888 => width * 4,
                psd_sprite::ColorType::A8 => width,
            } as usize;

            skia_safe::image::images::raster_from_data(
                &image_info,
                skia_safe::Data::new_copy(&pixels),
                row_bytes,
            )
            .map(|sk_image| {
                namui::Image::new(
                    namui::ImageInfo {
                        alpha_type: namui::AlphaType::Unpremul,
                        color_type: match header.color_type {
                            psd_sprite::ColorType::Rgba8888 => namui::ColorType::Rgba8888,
                            psd_sprite::ColorType::A8 => namui::ColorType::Alpha8,
                        },
                        height: (height as f32).px(),
                        width: (width as f32).px(),
                    },
                    sk_image,
                )
            })
            .ok_or(anyhow::anyhow!("Failed to create image from SpriteImage"))
        });

        sprite_image_loading_task.insert(sprite_image_id, task);
    }

    let sprite_loaded_images =
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
