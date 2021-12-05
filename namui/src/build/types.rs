use serde::Serialize;

#[derive(Serialize)]
pub struct ErrorMessage {
    pub relative_file: String,
    pub absolute_file: String,
    pub line: usize,
    pub column: usize,
    pub text: String,
}
