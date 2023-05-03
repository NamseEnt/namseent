use image::Rgba;
use libwebp_sys::WebPEncodeLosslessRGBA;
use namui_type::*;
use rayon::prelude::*;
use rpc::data::*;

struct ImageBuffer {
    id: Uuid,
    buffer: Vec<u8>,
    rect: Rect<Px>,
}

pub(crate) struct PsdParsingResult {
    pub(crate) variants_webps: Vec<(Uuid, Vec<u8>)>,
    pub(crate) cg_file: CgFile,
    pub(crate) cg_thumbnail_webp: Vec<u8>,
}

pub(crate) fn psd_to_webps_and_cg_file(
    psd_bytes: &[u8],
    filename: &str,
) -> Result<PsdParsingResult, psd::PsdError> {
    let psd = psd::Psd::from_bytes(psd_bytes)?;
    let mut parts: Vec<CgPart> = vec![];

    let mut image_buffers = vec![];

    for group_id in psd.group_ids_in_order() {
        let group = psd.groups().get(group_id).unwrap();
        let group_name = group.name();
        let selection_type = if group_name.ends_with("_s") {
            PartSelectionType::Single
        } else if group_name.ends_with("_m") {
            PartSelectionType::Multi
        } else {
            continue;
        };

        let mut variants = vec![];

        for layer in psd.layers() {
            if layer.parent_id() != Some(*group_id) {
                continue;
            }

            let layer_name = layer.name().to_string();

            let cropped = crop(
                layer.rgba(),
                Wh::new(psd.width() as usize, psd.height() as usize),
                Rect::Xywh {
                    x: layer.layer_left() as usize,
                    y: layer.layer_top() as usize,
                    width: layer.width() as usize,
                    height: layer.height() as usize,
                },
            );

            let id = namui_type::uuid_from_hash(format!(
                "{filename}.{parent_names}.{layer_name}",
                parent_names = concat_parent_names(&psd, layer.parent_id())
            ));

            let image_buffer = ImageBuffer {
                id,
                buffer: cropped,
                rect: Rect::Xywh {
                    x: layer.layer_left().px(),
                    y: layer.layer_top().px(),
                    width: (layer.width() as i32).px(),
                    height: (layer.height() as i32).px(),
                },
            };

            variants.push(CgPartVariant {
                name: layer_name,
                id,
                rect: Rect::Xywh {
                    x: ((layer.layer_left() as f32 / psd.width() as f32) * 100.0).percent(),
                    y: ((layer.layer_top() as f32 / psd.height() as f32) * 100.0).percent(),
                    width: ((layer.width() as f32 / psd.width() as f32) * 100.0).percent(),
                    height: ((layer.height() as f32 / psd.height() as f32) * 100.0).percent(),
                },
            });
            image_buffers.push(image_buffer);
        }

        for (group_id, group) in psd.groups() {
            if group.parent_id() != Some(*group_id) {
                continue;
            }

            let name = group.name().to_string();

            let id = namui_type::uuid_from_hash(format!(
                "{filename}.{parent_names}.{name}",
                parent_names = concat_parent_names(&psd, group.parent_id())
            ));

            let image_buffer = ImageBuffer {
                id,
                buffer: psd
                    .flatten_layers_rgba(&|(_, layer)| layer.parent_id() == Some(*group_id))?,
                rect: Rect::Xywh {
                    x: 0.px(),
                    y: 0.px(),
                    width: (psd.width() as i32).px(),
                    height: (psd.height() as i32).px(),
                },
            };

            variants.push(CgPartVariant {
                name,
                id,
                rect: Rect::Xywh {
                    x: 0.percent(),
                    y: 0.percent(),
                    width: 100.percent(),
                    height: 100.percent(),
                },
            });
            image_buffers.push(image_buffer);
        }

        parts.push(CgPart {
            name: concat_parent_names(&psd, group.parent_id()) + group.name(),
            selection_type,
            variants,
        });
    }

    let webps = image_buffers
        .par_iter()
        .map(
            |&ImageBuffer {
                 id,
                 ref buffer,
                 rect,
             }| {
                let webp = encode_rgba_webp(&buffer, rect.width().into(), rect.height().into());
                (id, webp)
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
                            if image_buffer.id == variant.id {
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
        encode_rgba_webp(
            &merged_image,
            (psd.width() as i32).px(),
            (psd.height() as i32).px(),
        )
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

fn merge_images(part_images: Vec<&ImageBuffer>, psd_width: usize, psd_height: usize) -> Vec<u8> {
    let mut bottom =
        image::ImageBuffer::<image::Rgba<u8>, _>::new(psd_width as u32, psd_height as u32);

    for part_image in part_images.into_iter() {
        let part_iamge_width = part_image.rect.width().as_f32() as u32;
        let part_iamge_height = part_image.rect.height().as_f32() as u32;

        assert_eq!(
            part_iamge_width * part_iamge_height * 4,
            part_image.buffer.len() as u32
        );

        let image = image::ImageBuffer::from_vec(
            part_image.rect.width().as_f32() as u32,
            part_image.rect.height().as_f32() as u32,
            part_image.buffer.clone(),
        )
        .unwrap();

        image::imageops::overlay(
            &mut bottom,
            &image,
            part_image.rect.x().as_f32() as i64,
            part_image.rect.y().as_f32() as i64,
        );
    }

    bottom.into_raw()
}

fn encode_rgba_webp(input_image: &[u8], width: Px, height: Px) -> Vec<u8> {
    unsafe {
        let mut out_buf = std::ptr::null_mut();
        let stride = width.as_f32() as i32 * 4;
        let len = WebPEncodeLosslessRGBA(
            input_image.as_ptr(),
            width.as_f32() as i32,
            height.as_f32() as i32,
            stride,
            &mut out_buf,
        );
        std::slice::from_raw_parts(out_buf, len as usize).into()
    }
}

pub fn crop(input_image: Vec<u8>, source_wh: Wh<usize>, crop_rect: Rect<usize>) -> Vec<u8> {
    let mut buffer: image::ImageBuffer<Rgba<u8>, _> =
        image::ImageBuffer::from_raw(source_wh.width as u32, source_wh.height as u32, input_image)
            .unwrap();

    let cropped = image::imageops::crop(
        &mut buffer,
        crop_rect.x() as u32,
        crop_rect.y() as u32,
        crop_rect.width() as u32,
        crop_rect.height() as u32,
    );

    cropped.to_image().into_raw()
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
