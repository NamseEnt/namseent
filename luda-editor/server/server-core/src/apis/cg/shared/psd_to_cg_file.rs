use super::{
    layer_tree::{render_layer_tree, RenderResult},
    parse_psd_to_inter_cg_parts::InterCgVariant,
    *,
};
use namui_type::*;
use rayon::prelude::*;
use rpc::data::*;

pub(crate) struct PsdParsingResult {
    pub(crate) variants_webps: Vec<(Uuid, Vec<u8>)>,
    pub(crate) cg_file: CgFile,
    pub(crate) cg_thumbnail_webp: Vec<u8>,
}

pub(crate) struct VariantImageBuffer {
    pub(crate) variant_id: Uuid,
    pub(crate) image_buffer: image::ImageBuffer<image::Rgba<u8>, Vec<u8>>,
    pub(crate) xy: Xy<Px>,
}

pub(crate) fn psd_to_webps_and_cg_file(
    psd_bytes: &[u8],
    filename: &str,
) -> Result<PsdParsingResult, psd::PsdError> {
    let psd = psd::Psd::from_bytes(psd_bytes)?;

    let inter_cg_parts = parse_psd_to_inter_cg_parts::parse_psd_to_inter_cg_parts(&psd);

    let (parts, image_buffers) = inter_cg_parts
        .into_par_iter()
        .map(|inter_cg_part| {
            let (variants, image_buffers) = inter_cg_part
                .variants
                .into_par_iter()
                .map(|inter_cg_variant| {
                    inter_cg_variant_to_cg_variant_and_image_buffer(
                        filename,
                        &psd,
                        inter_cg_variant,
                    )
                })
                .unzip::<_, _, Vec<_>, Vec<_>>();

            (
                CgPart {
                    name: inter_cg_part.part_name,
                    selection_type: inter_cg_part.selection_type,
                    variants,
                },
                image_buffers,
            )
        })
        .unzip::<_, _, Vec<_>, Vec<_>>();

    let image_buffers = image_buffers.into_iter().flatten().collect::<Vec<_>>();
    let webps = image_buffers
        .par_iter()
        .map(
            |&VariantImageBuffer {
                 variant_id,
                 ref image_buffer,
                 ..
             }| {
                let webp =
                    encode_rgba_webp(&image_buffer, image_buffer.width(), image_buffer.height());
                (variant_id, webp)
            },
        )
        .collect::<Vec<_>>();

    let cg_thumbnail_webp = {
        let part_images = parts
            .iter()
            .rev()
            .filter_map(|part| {
                part.variants.first().map(|variant| {
                    image_buffers
                        .iter()
                        .find_map(|image_buffer| {
                            if image_buffer.variant_id == variant.id {
                                Some(image_buffer)
                            } else {
                                None
                            }
                        })
                        .unwrap()
                })
            })
            .collect();
        let merged_image = merge_images(part_images, psd.width() as usize, psd.height() as usize);
        encode_rgba_webp(&merged_image, psd.width(), psd.height())
    };

    Ok(PsdParsingResult {
        cg_file: CgFile {
            id: namui_type::uuid_from_hash(filename),
            name: filename.to_string(),
            parts,
            width_per_height: Per::new((psd.width() as i32).px(), (psd.height() as i32).px()),
        },
        variants_webps: webps,
        cg_thumbnail_webp,
    })
}

fn merge_images(
    part_images: Vec<&VariantImageBuffer>,
    psd_width: usize,
    psd_height: usize,
) -> Vec<u8> {
    let mut bottom =
        image::ImageBuffer::<image::Rgba<u8>, _>::new(psd_width as u32, psd_height as u32);

    for part_image in part_images.into_iter() {
        let part_iamge_width = part_image.image_buffer.width();
        let part_iamge_height = part_image.image_buffer.height();

        assert_eq!(
            part_iamge_width * part_iamge_height * 4,
            part_image.image_buffer.len() as u32
        );

        image::imageops::overlay(
            &mut bottom,
            &part_image.image_buffer,
            part_image.xy.x.as_f32() as i64,
            part_image.xy.y.as_f32() as i64,
        );
    }

    bottom.into_raw()
}

fn encode_rgba_webp(input_image: &[u8], width: u32, height: u32) -> Vec<u8> {
    let mut out_buf = vec![];

    let encoder = image::codecs::webp::WebPEncoder::new(&mut out_buf);
    encoder
        .encode(input_image, width, height, image::ColorType::Rgba8)
        .unwrap();

    out_buf
}

fn inter_cg_variant_to_cg_variant_and_image_buffer(
    filename: &str,
    psd: &psd::Psd,
    inter_cg_variant: InterCgVariant,
) -> (CgPartVariant, VariantImageBuffer) {
    let id = namui_type::uuid_from_hash(format!(
        "{filename}.{part_name}.{variant_name}",
        part_name = inter_cg_variant.part_name,
        variant_name = inter_cg_variant.variant_name,
    ));

    let width = psd.width();
    let height = psd.height();

    let RenderResult { x, y, image_buffer } =
        render_layer_tree(psd, &inter_cg_variant.layer_tree, true);

    (
        CgPartVariant {
            id,
            name: inter_cg_variant.variant_name,
            rect: Rect::Xywh {
                x: (100.0 * x as f32 / width as f32).percent(),
                y: (100.0 * y as f32 / height as f32).percent(),
                width: (100.0 * image_buffer.width() as f32 / width as f32).percent(),
                height: (100.0 * image_buffer.height() as f32 / height as f32).percent(),
            },
            blend_mode: match inter_cg_variant.blend_mode {
                psd::BlendMode::PassThrough => CgPartVariantBlendMode::PassThrough,
                psd::BlendMode::Normal => CgPartVariantBlendMode::Normal,
                psd::BlendMode::Dissolve => CgPartVariantBlendMode::Dissolve,
                psd::BlendMode::Darken => CgPartVariantBlendMode::Darken,
                psd::BlendMode::Multiply => CgPartVariantBlendMode::Multiply,
                psd::BlendMode::ColorBurn => CgPartVariantBlendMode::ColorBurn,
                psd::BlendMode::LinearBurn => CgPartVariantBlendMode::LinearBurn,
                psd::BlendMode::DarkerColor => CgPartVariantBlendMode::DarkerColor,
                psd::BlendMode::Lighten => CgPartVariantBlendMode::Lighten,
                psd::BlendMode::Screen => CgPartVariantBlendMode::Screen,
                psd::BlendMode::ColorDodge => CgPartVariantBlendMode::ColorDodge,
                psd::BlendMode::LinearDodge => CgPartVariantBlendMode::LinearDodge,
                psd::BlendMode::LighterColor => CgPartVariantBlendMode::LighterColor,
                psd::BlendMode::Overlay => CgPartVariantBlendMode::Overlay,
                psd::BlendMode::SoftLight => CgPartVariantBlendMode::SoftLight,
                psd::BlendMode::HardLight => CgPartVariantBlendMode::HardLight,
                psd::BlendMode::VividLight => CgPartVariantBlendMode::VividLight,
                psd::BlendMode::LinearLight => CgPartVariantBlendMode::LinearLight,
                psd::BlendMode::PinLight => CgPartVariantBlendMode::PinLight,
                psd::BlendMode::HardMix => CgPartVariantBlendMode::HardMix,
                psd::BlendMode::Difference => CgPartVariantBlendMode::Difference,
                psd::BlendMode::Exclusion => CgPartVariantBlendMode::Exclusion,
                psd::BlendMode::Subtract => CgPartVariantBlendMode::Subtract,
                psd::BlendMode::Divide => CgPartVariantBlendMode::Divide,
                psd::BlendMode::Hue => CgPartVariantBlendMode::Hue,
                psd::BlendMode::Saturation => CgPartVariantBlendMode::Saturation,
                psd::BlendMode::Color => CgPartVariantBlendMode::Color,
                psd::BlendMode::Luminosity => CgPartVariantBlendMode::Luminosity,
            },
        },
        VariantImageBuffer {
            variant_id: id,
            image_buffer,
            xy: Xy::new((x as f32).px(), (y as f32).px()),
        },
    )
}
