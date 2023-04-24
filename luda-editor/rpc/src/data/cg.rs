#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CgFile {
    pub name: String,
    pub parts: Vec<CgPart>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CgPart {
    pub name: String,
    pub selection_type: PartSelecitonType,
    pub variants: Vec<CgPartVariant>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CgPartVariant {
    pub name: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum PartSelecitonType {
    Single,
    Multi,
}
