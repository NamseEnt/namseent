use regex::Regex;

#[derive(PartialEq, Eq, Debug)]
pub struct Path {
    pub elements: Vec<PathElement>,
}

#[derive(Debug)]
pub enum PathElement {
    FileOrDir { raw_string: String, regex: Regex },
    DoubleAsterisk,
    CurrentDirectory,
    ParentDirectory,
}
impl PartialEq for PathElement {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (
                Self::FileOrDir {
                    raw_string: l_raw_string,
                    regex: l_regex,
                },
                Self::FileOrDir {
                    raw_string: r_raw_string,
                    regex: r_regex,
                },
            ) => l_raw_string == r_raw_string && l_regex.as_str() == r_regex.as_str(),
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}
impl Eq for PathElement {}
