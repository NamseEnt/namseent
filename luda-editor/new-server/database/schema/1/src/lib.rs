#[schema_macro::schema]
pub struct MyDocument {
    pub name: String,
    pub content: String,
    pub tags: Vec<String>,
}
