use super::*;
use crate::data::ScreenImage;
use crate::Uuid;

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
    pub fn migrate(previous: v0::Cut) -> Self {
        Self {
            id: previous.id(),
            line: previous.line,
            character_id: previous.character_id,
            screen_images: previous
                .screen_image_ids
                .map(|image_id| image_id.map(ScreenImage::new)),
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
            screen_images: self.screen_images,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_migrate() {
        let previous = v0::Cut::new(Uuid::new_v4());
        let previous_id = previous.id();
        let current = Cut::migrate(previous);
        assert_eq!(current.id(), previous_id);
    }

    #[test]
    fn test_serialize() {
        let id = Uuid::new_v4();
        let cut = Cut::new(id);

        let serialized = bincode::serialize(&cut).unwrap();
        let expected = std::iter::empty::<u8>()
            // id (24 bytes): uuid length (8 bytes) + uuid (16 bytes)
            .chain([16, 0, 0, 0, 0, 0, 0, 0]) // uuid length (8 bytes)
            .chain(*id.as_bytes()) // uuid (16 bytes)
            // line (8 bytes): string length (8 bytes) + string (0 bytes)
            .chain([0, 0, 0, 0, 0, 0, 0, 0]) // string length (8 bytes)
            // character_id (1 bytes): none (1 byte)
            .chain([0]) // none (1 byte)
            // screen_images (5 bytes): none (1 byte) * 5
            .chain([0, 0, 0, 0, 0]) // none * 5
            .collect::<Vec<_>>();

        assert_eq!(serialized, expected);
    }

    #[test]
    fn test_deserialize() {
        let id = Uuid::new_v4();
        let bytes = std::iter::empty::<u8>()
            // id (24 bytes): uuid length (8 bytes) + uuid (16 bytes)
            .chain([16, 0, 0, 0, 0, 0, 0, 0]) // uuid length (8 bytes)
            .chain(*id.as_bytes()) // uuid (16 bytes)
            // line (8 bytes): string length (8 bytes) + string (0 bytes)
            .chain([0, 0, 0, 0, 0, 0, 0, 0]) // string length (8 bytes)
            // character_id (1 bytes): none (1 byte)
            .chain([0]) // none (1 byte)
            // screen_images (5 bytes): none (1 byte) * 5
            .chain([0, 0, 0, 0, 0]) // none * 5
            .collect::<Vec<_>>();

        let cut: Cut = migration::Migration::deserialize(&bytes, 1).unwrap();
        assert_eq!(cut.id(), id);
    }

    #[test]
    fn test_deserialize_migrate() {
        let id = Uuid::new_v4();
        // id: Uuid,
        // /// The text that the character speaks in this cut.
        // pub line: String,
        // pub character_id: Option<Uuid>,
        // pub screen_image_ids: [Option<Uuid>; 5],
        let bytes = std::iter::empty::<u8>()
            // id (24 bytes): uuid length (8 bytes) + uuid (16 bytes)
            .chain([16, 0, 0, 0, 0, 0, 0, 0]) // uuid length (8 bytes)
            .chain(*id.as_bytes()) // uuid (16 bytes)
            // line (8 bytes): string length (8 bytes) + string (0 bytes)
            .chain([0, 0, 0, 0, 0, 0, 0, 0]) // string length (8 bytes)
            // character_id (1 bytes): none (1 byte)
            .chain([0]) // none (1 byte)
            // screen_image_ids (5 bytes): none (1 byte) * 5
            .chain([0, 0, 0, 0, 0]) // none * 5
            .collect::<Vec<_>>();

        let cut: Cut = migration::Migration::deserialize(&bytes, 0).unwrap();
        assert_eq!(cut.id(), id);
    }
}
