mod parse_psd_to_inter_cg_parts;

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

    let mut bottom = image::ImageBuffer::<image::Rgba<u8>, _>::new(width, height);
    let rect: Rect<Percent> = {
        let rect_in_pixel = inter_cg_variant.layers.iter().fold(
            Rect::<i32>::Ltrb {
                left: width as i32,
                top: height as i32,
                right: 0,
                bottom: 0,
            },
            |acc, layer| {
                let rect = Rect::Xywh {
                    x: layer.layer_left(),
                    y: layer.layer_top(),
                    width: layer.width() as i32,
                    height: layer.height() as i32,
                };
                acc.get_minimum_rectangle_containing(rect)
            },
        );
        Rect::Xywh {
            x: (100.0 * rect_in_pixel.x() as f32 / width as f32).percent(),
            y: (100.0 * rect_in_pixel.y() as f32 / height as f32).percent(),
            width: (100.0 * rect_in_pixel.width() as f32 / width as f32).percent(),
            height: (100.0 * rect_in_pixel.height() as f32 / height as f32).percent(),
        }
    };

    let images = inter_cg_variant
        .layers
        .into_par_iter()
        .map(|layer| image::ImageBuffer::from_vec(width, height, layer.rgba()).unwrap())
        .collect::<Vec<_>>();

    for image in images.into_iter().rev() {
        image::imageops::overlay(&mut bottom, &image, 0, 0);
    }

    (
        CgPartVariant {
            id,
            name: inter_cg_variant.variant_name,
            rect,
        },
        VariantImageBuffer {
            variant_id: id,
            image_buffer: bottom.into(),
        },
    )
}

fn concat_parent_names(psd: &psd::Psd, mut parent_group_id: Option<u32>) -> String {
    let mut parent_names = vec![];

    while let Some(group_id) = parent_group_id {
        let (_, parent_group) = psd
            .groups()
            .into_iter()
            .find(|(x, _)| **x == group_id)
            .unwrap();
        parent_names.insert(0, parent_group.name().to_string());
        parent_group_id = parent_group.parent_id();
    }

    parent_names.join(".")
}
