use namui_type::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CgFile {
    pub id: Uuid,
    pub name: String,
    pub parts: Vec<CgPart>,
    pub width_per_height: namui_type::Per<Px, Px>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CgPart {
    pub name: String,
    pub selection_type: PartSelecitonType,
    pub variants: Vec<CgPartVariant>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CgPartVariant {
    pub id: Uuid,
    pub name: String,
    pub rect: Rect<Percent>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum PartSelecitonType {
    Single,
    Multi,
}
