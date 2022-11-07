use super::previous;
use ::uuid::Uuid;
use namui_type::*;

pub type ScreenImages = [Option<ScreenImage>; 5];

#[migration::version(1)]
#[derive(Debug, Clone, Default)]
pub struct Cut {
    id: Uuid,
    /// The text that the character speaks in this cut.
    pub line: String,
    pub character_id: Option<Uuid>,
    pub screen_images: ScreenImages,
}

impl Cut {
    pub fn migrate(previous: previous::v0::Cut) -> Self {
        Self {
            id: previous.id(),
            line: previous.line,
            character_id: previous.character_id,
            screen_images: previous
                .screen_image_ids
                .map(|image_id| image_id.map(|image_id| ScreenImage::new(image_id))),
        }
    }
    pub fn new(id: Uuid) -> Self {
        Self {
            id,
            line: String::new(),
            character_id: None,
            screen_images: [None, None, None, None, None],
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
            screen_images: self.screen_images.clone(),
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ScreenImage {
    pub id: Uuid,
    pub circumscribed: Circumscribed<Percent>,
}
impl ScreenImage {
    pub fn new(id: Uuid) -> Self {
        Self {
            id,
            circumscribed: Circumscribed {
                center_xy: Xy::new(50.percent(), 50.percent()),
                radius: 50.percent(),
            },
        }
    }
}

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub struct Circumscribed<T> {
    /// (0,0) : left top, (1,1) : right bottom
    pub center_xy: Xy<T>,
    /// 1.0 = 100% of the screen's radius
    pub radius: T,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_migrate() {
        let previous = previous::v0::Cut::new(Uuid::new_v4());
        let previous_id = previous.id();
        let current = Cut::migrate(previous);
        assert_eq!(current.id(), previous_id);
    }

    #[test]
    fn test_serialize() {
        let id = Uuid::new_v4();
        let cut = Cut::new(id);
        let json = serde_json::to_string(&cut).unwrap();
        assert_eq!(
            json,
            format!(
                r#"{{"_v":1,"id":"{}","line":"","character_id":null,"screen_images":[null,null,null,null,null]}}"#,
                id
            )
        );
    }

    #[test]
    fn test_deserialize() {
        let id = Uuid::new_v4();
        let json = format!(
            r#"{{"_v":1,"id":"{}","line":"","character_id":null,"screen_images":[null,null,null,null,null]}}"#,
            id
        );
        let cut: Cut = serde_json::from_str(&json).unwrap();
        assert_eq!(cut.id(), id);
    }

    #[test]
    fn test_deserialize_migrate() {
        let id = Uuid::new_v4();
        let json = format!(
            r#"{{"id":"{}","line":"","character_id":null,"screen_image_ids":[null,null,null,null,null]}}"#,
            id
        );
        let cut: Cut = serde_json::from_str(&json).unwrap();
        assert_eq!(cut.id(), id);
    }
}
