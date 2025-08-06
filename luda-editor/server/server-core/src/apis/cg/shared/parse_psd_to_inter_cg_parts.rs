use super::layer_tree::{make_tree, LayerTree};
use psd::BlendMode;
use rpc::data::PartSelectionType;

pub struct InterCgPart<'a> {
    pub part_name: String,
    pub selection_type: PartSelectionType,
    pub variants: Vec<InterCgVariant<'a>>,
}

pub struct InterCgVariant<'a> {
    pub part_name: String,
    pub variant_name: String,
    pub layer_tree: Vec<LayerTree<'a>>,
    pub blend_mode: BlendMode,
}

pub fn parse_psd_to_inter_cg_parts(psd: &psd::Psd) -> Vec<InterCgPart<'_>> {
    let layer_tree = make_tree(psd).expect("Failed to make tree");
    let parts = create_inter_cg_part_from_layer_tree(layer_tree, vec![], BlendMode::Normal);
    parts
}

fn create_inter_cg_part_from_layer_tree(
    layer_tree: Vec<LayerTree<'_>>,
    layer_full_names: Vec<String>,
    blend_mode: BlendMode,
) -> Vec<InterCgPart<'_>> {
    let mut merging_layer_tree = vec![];
    let mut parts = vec![];

    fn push_merging_layer_tree_as_always_on_part<'psd>(
        merging_layer_tree: &mut Vec<LayerTree<'psd>>,
        parts: &mut Vec<InterCgPart<'psd>>,
        layer_full_names: &[String],
        blend_mode: BlendMode,
    ) {
        if merging_layer_tree.is_empty() {
            return;
        }
        let part_name = layer_full_names.join(".");
        let variant_name = merging_layer_tree
            .iter()
            .map(|layer_tree| layer_tree.name())
            .fold(String::new(), |acc, name| format!("{acc}_{name}"));
        let layer_tree = {
            let mut result = vec![];
            result.append(merging_layer_tree);
            result
        };
        parts.push(InterCgPart {
            part_name: part_name.clone(),
            selection_type: PartSelectionType::AlwaysOn,
            variants: vec![InterCgVariant {
                part_name,
                variant_name,
                layer_tree,
                blend_mode,
            }],
        });
    }

    for layer_tree in layer_tree {
        if layer_tree.has_no_selection() {
            merging_layer_tree.push(layer_tree);
            continue;
        }
        push_merging_layer_tree_as_always_on_part(
            &mut merging_layer_tree,
            &mut parts,
            &layer_full_names,
            blend_mode,
        );
        match layer_tree {
            LayerTree::Group { item, children } => {
                let name = item.name();
                if name.ends_with("_s") || name.ends_with("_m") {
                    let selection_type = if name.ends_with("_s") {
                        PartSelectionType::Single
                    } else {
                        PartSelectionType::Multi
                    };
                    let mut layer_full_names = layer_full_names.clone();
                    layer_full_names.push(name.to_string());
                    let part_name = layer_full_names.join(".");
                    let mut variants = vec![];
                    let mut layer_tree = vec![];
                    for child in children {
                        let clipping = !child.is_clipping();
                        let variant_name = child.name().to_string();
                        let blend_mode = child.blend_mode();
                        layer_tree.push(child);
                        if clipping {
                            continue;
                        }
                        variants.push(InterCgVariant {
                            part_name: part_name.clone(),
                            variant_name,
                            layer_tree,
                            blend_mode,
                        });
                        layer_tree = vec![];
                    }

                    parts.push(InterCgPart {
                        part_name,
                        selection_type,
                        variants,
                    });
                    continue;
                }

                let mut layer_full_names = layer_full_names.clone();
                layer_full_names.push(name.to_string());
                parts.append(&mut create_inter_cg_part_from_layer_tree(
                    children,
                    layer_full_names,
                    blend_mode,
                ));
            }
            LayerTree::Layer { .. } => {
                unreachable!("It might be group")
            }
        }
    }
    push_merging_layer_tree_as_always_on_part(
        &mut merging_layer_tree,
        &mut parts,
        &layer_full_names,
        blend_mode,
    );

    parts
}
