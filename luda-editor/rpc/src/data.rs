#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct ProjectSharedData {
    id: String,
    pub characters: Vec<Character>,
}
impl ProjectSharedData {
    pub fn new(id: String) -> Self {
        Self {
            id,
            characters: vec![],
        }
    }
    #[allow(dead_code)]
    pub fn id(&self) -> &str {
        &self.id
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct Sequence {
    id: String,
    pub name: String,
    pub cuts: Vec<Cut>,
}
impl Sequence {
    pub fn new(id: String, name: String) -> Self {
        Self {
            id,
            name,
            cuts: Vec::new(),
        }
    }
    #[allow(dead_code)]
    pub fn id(&self) -> &str {
        &self.id
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct Cut {
    id: String,
    /// The text that the character speaks in this cut.
    pub line: String,
    pub character_id: Option<String>,
    pub face_expression_id: Option<String>,
}

impl Cut {
    pub fn new(id: String) -> Self {
        Self {
            id,
            line: String::new(),
            character_id: None,
            face_expression_id: None,
        }
    }
    pub fn id(&self) -> &str {
        &self.id
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct Character {
    id: String,
    pub name: String,
}

#[allow(dead_code)]
impl Character {
    pub fn new(id: String) -> Self {
        Self {
            id,
            name: String::new(),
        }
    }
    pub fn id(&self) -> &str {
        &self.id
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct Label {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct UrlWithLabels {
    pub url: String,
    pub labels: Vec<Label>,
}

// TODO: implement migration
