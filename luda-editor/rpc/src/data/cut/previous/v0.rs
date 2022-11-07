use uuid::Uuid;

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct Cut {
    id: Uuid,
    /// The text that the character speaks in this cut.
    pub line: String,
    pub character_id: Option<Uuid>,
    pub screen_image_ids: [Option<Uuid>; 5],
}

impl Cut {
    pub fn new(id: Uuid) -> Self {
        Self {
            id,
            line: String::new(),
            character_id: None,
            screen_image_ids: [None; 5],
        }
    }
    pub fn id(&self) -> Uuid {
        self.id
    }
    pub fn duplicate(&self, id: Uuid) -> Self {
        Self {
            id,
            line: self.line.clone(),
            character_id: self.character_id,
            screen_image_ids: self.screen_image_ids,
        }
    }
}
