use rpc::data::PartSelectionType;

pub struct InterCgPart<'a> {
    pub part_name: String,
    pub selection_type: PartSelectionType,
    pub variants: Vec<InterCgVariant<'a>>,
}

pub struct InterCgVariant<'a> {
    pub part_name: String,
    pub variant_name: String,
    pub layers: Vec<&'a psd::PsdLayer>,
}

pub fn parse_psd_to_inter_cg_parts<'a>(psd: &'a psd::Psd) -> Vec<InterCgPart<'a>> {
    let mut parts: Vec<InterCgPart<'a>> = vec![];
    for layer in psd.layers() {
        let full_names = layer_full_names(psd, layer);

        let (split_index, selection_type) = full_names
            .iter()
            .enumerate()
            .find_map(|(index, name)| {
                if name.ends_with("_s") {
                    Some((index, PartSelectionType::Single))
                } else if name.ends_with("_m") {
                    Some((index, PartSelectionType::Multi))
                } else if full_names.len() - 1 == index {
                    Some((index, PartSelectionType::AlwaysOn))
                } else {
                    None
                }
            })
            .expect(format!("fail to parse psd {:?}", full_names).as_str());

        let part_name = full_names[..split_index + 1].join(".");
        let variant_name = full_names
            .get(split_index + 1)
            .unwrap_or(&"".to_string())
            .to_string();

        if let Some(part) = parts.iter_mut().find(|x| x.part_name == part_name) {
            if let Some(variant) = part
                .variants
                .iter_mut()
                .find(|x| x.variant_name == variant_name)
            {
                variant.layers.push(layer);
            } else {
                part.variants.push(InterCgVariant {
                    part_name,
                    variant_name,
                    layers: vec![layer],
                });
            }
        } else {
            parts.push(InterCgPart {
                part_name: part_name.clone(),
                selection_type,
                variants: vec![InterCgVariant {
                    part_name,
                    variant_name,
                    layers: vec![layer],
                }],
            });
        }
    }

    parts
}

fn layer_full_names(psd: &psd::Psd, layer: &psd::PsdLayer) -> Vec<String> {
    let mut parent_group_id = layer.parent_id();
    let mut group_names = vec![layer.name().to_string()];

    while let Some(group_id) = parent_group_id {
        let (_, parent_group) = psd
            .groups()
            .into_iter()
            .find(|(x, _)| **x == group_id)
            .unwrap();
        group_names.push(parent_group.name().to_string());
        parent_group_id = parent_group.parent_id();
    }

    group_names.reverse();
    group_names
}
