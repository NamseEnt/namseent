use msoffice_pptx::{document::PPTXDocument, pml::SlideIdListEntry};
use msoffice_shared::xml::XmlNode;
use std::{
    collections::HashMap,
    fs::File,
    path::{Path, PathBuf},
    str::FromStr,
};
use zip::ZipArchive;

pub fn get_slide_path_list_in_order(
    zipper: &mut ZipArchive<File>,
    ppt: &PPTXDocument,
) -> Vec<PathBuf> {
    let mut slide_path_list = Vec::new();
    let ppt_rel_map = get_ppt_rel_map(zipper);
    for SlideIdListEntry {
        relationship_id, ..
    } in ppt.presentation.as_ref().unwrap().slide_id_list.iter()
    {
        slide_path_list.push(ppt_rel_map.get(relationship_id).unwrap().to_path_buf());
    }
    slide_path_list
}

fn get_ppt_rel_map(zipper: &mut ZipArchive<File>) -> HashMap<String, PathBuf> {
    let mut ppt_rel_map = HashMap::new();
    let xml_string =
        std::io::read_to_string(&mut zipper.by_name("ppt/_rels/presentation.xml.rels").unwrap())
            .unwrap();
    let xml_node = XmlNode::from_str(xml_string.as_str()).unwrap();

    for child_node in &xml_node.child_nodes {
        let id = child_node.attributes.get("Id").cloned().unwrap();
        let target = Path::new("ppt")
            .join(PathBuf::from_str(child_node.attributes.get("Target").unwrap()).unwrap());
        ppt_rel_map.insert(id, target);
    }
    ppt_rel_map
}
