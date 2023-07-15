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
    pub blend_mode: CgPartVariantBlendMode,
}

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub enum CgPartVariantBlendMode {
    PassThrough,
    Normal,
    Dissolve,
    Darken,
    Multiply,
    ColorBurn,
    LinearBurn,
    DarkerColor,
    Lighten,
    Screen,
    ColorDodge,
    LinearDodge,
    LighterColor,
    Overlay,
    SoftLight,
    HardLight,
    VividLight,
    LinearLight,
    PinLight,
    HardMix,
    Difference,
    Exclusion,
    Subtract,
    Divide,
    Hue,
    Saturation,
    Color,
    Luminosity,
}

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize, PartialEq)]
pub enum PartSelectionType {
    AlwaysOn,
    Single,
    Multi,
}
