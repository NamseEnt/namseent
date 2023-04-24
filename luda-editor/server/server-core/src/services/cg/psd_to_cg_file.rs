use libwebp_sys::WebPEncodeLosslessRGBA;
use rayon::prelude::*;
use rpc::data::*;

pub(crate) fn psd_to_webps_and_cg_file(
    psd_bytes: &[u8],
    filename: &str,
) -> Result<(Vec<(String, Vec<u8>)>, CgFile), psd::PsdError> {
    let psd = psd::Psd::from_bytes(psd_bytes)?;
    let mut parts: Vec<CgPart> = vec![];

    struct ImageBuffer {
        name: String,
        buffer: Vec<u8>,
        width: usize,
        height: usize,
    }

    let mut image_buffers = vec![];

    for (group_id, group) in psd.groups() {
        let group_name = group.name();
        let selection_type = if group_name.ends_with("_s") {
            PartSelecitonType::Single
        } else if group_name.ends_with("_m") {
            PartSelecitonType::Multi
        } else {
            continue;
        };

        let mut variants = vec![];

        for layer in psd.layers() {
            if layer.parent_id() != Some(*group_id) {
                continue;
            }

            let name = layer.name().to_string();

            let image_buffer = ImageBuffer {
                name: name.clone(),
                buffer: layer.rgba().to_vec(),
                width: layer.width() as usize,
                height: layer.height() as usize,
            };

            variants.push(CgPartVariant { name });
            image_buffers.push(image_buffer);
        }

        for (group_id, group) in psd.groups() {
            if group.parent_id() != Some(*group_id) {
                continue;
            }

            let name = group.name().to_string();

            let image_buffer = ImageBuffer {
                name: name.clone(),
                buffer: psd
                    .flatten_layers_rgba(&|(_, layer)| layer.parent_id() == Some(*group_id))?,
                width: psd.width() as usize,
                height: psd.height() as usize,
            };

            variants.push(CgPartVariant { name });
            image_buffers.push(image_buffer);
        }

        parts.push(CgPart {
            name: concat_parent_names(&psd, group.parent_id()) + group.name(),
            selection_type,
            variants,
        });
    }

    let webps = image_buffers
        .into_par_iter()
        .map(
            |ImageBuffer {
                 name,
                 buffer,
                 width,
                 height,
             }| {
                let webp = encode_rgba_webp(&buffer, width, height);
                (name, webp)
            },
        )
        .collect::<Vec<_>>();

    Ok((
        webps,
        CgFile {
            name: filename.to_string(),
            parts,
        },
    ))
}

fn encode_rgba_webp(input_image: &[u8], width: usize, height: usize) -> Vec<u8> {
    unsafe {
        let mut out_buf = std::ptr::null_mut();
        let stride = width as i32 * 4;
        let len = WebPEncodeLosslessRGBA(
            input_image.as_ptr(),
            width as i32,
            height as i32,
            stride,
            &mut out_buf,
        );
        std::slice::from_raw_parts(out_buf, len as usize).into()
    }
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
