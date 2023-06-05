use namui_type::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CgFile {
    /// TODO: Rename id as checksum. real id is name.
    pub id: Uuid,
    pub name: String,
    pub parts: Vec<CgPart>,
    pub width_per_height: namui_type::Per<Px, Px>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CgPart {
    pub name: String,
    pub selection_type: PartSelectionType,
    pub variants: Vec<CgPartVariant>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CgPartVariant {
    /// TODO: Rename id as checksum. real id is name.
    pub id: Uuid,
    pub name: String,
    pub rect: Rect<Percent>,
}

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize, PartialEq)]
pub enum PartSelectionType {
    AlwaysOn,
    Single,
    Multi,
}
