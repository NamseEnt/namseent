mod layer_tree;
mod parse_psd_to_inter_cg_parts;

use self::layer_tree::render_layer_tree;
use self::layer_tree::RenderResult;
use self::parse_psd_to_inter_cg_parts::InterCgVariant;
use crate::psd_parsing::parse_psd_to_inter_cg_parts::parse_psd_to_inter_cg_parts;
use namui_type::*;
use rayon::prelude::*;
use rpc::data::*;
use rpc::Uuid;

pub(crate) struct PsdParsingResult {
    pub(crate) variants_images: Vec<VariantImageBuffer>,
    pub(crate) cg_file: CgFile,
    pub(crate) wh: Wh<u32>,
}

pub(crate) struct VariantImageBuffer {
    pub(crate) variant_id: Uuid,
    pub(crate) image_buffer: image::ImageBuffer<image::Rgba<u8>, Vec<u8>>,
    pub(crate) rect: Rect<i32>,
}

pub(crate) fn parse_psd(
    psd_bytes: &[u8],
    filename: &str,
) -> Result<PsdParsingResult, psd::PsdError> {
    let psd = psd::Psd::from_bytes(psd_bytes)?;

    let inter_cg_parts = parse_psd_to_inter_cg_parts(&psd);

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

    let width = psd.width();
    let height = psd.height();

    Ok(PsdParsingResult {
        cg_file: CgFile {
            id: namui_type::uuid_from_hash(filename),
            name: filename.to_string(),
            parts,
            width_per_height: Per::new((width as i32).px(), (height as i32).px()),
        },
        variants_images: image_buffers,
        wh: Wh::new(width, height),
    })
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
        },
        VariantImageBuffer {
            variant_id: id,
            rect: Rect::Xywh {
                x,
                y,
                width: image_buffer.width() as i32,
                height: image_buffer.height() as i32,
            },
            image_buffer,
        },
    )
}
