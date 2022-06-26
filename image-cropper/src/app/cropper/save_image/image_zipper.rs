use std::io::{Cursor, Write};
use zip::ZipWriter;

pub struct ImageZipper {
    writer: ZipWriter<Cursor<Vec<u8>>>,
}
impl ImageZipper {
    pub fn new() -> Self {
        Self {
            writer: ZipWriter::new(Cursor::new(Vec::new())),
        }
    }

    pub fn add_image(&mut self, name: &str, image: Vec<u8>) {
        self.writer
            .start_file(name, zip::write::FileOptions::default())
            .unwrap();
        self.writer.write_all(image.as_ref()).unwrap();
    }

    pub fn finish(mut self) -> Vec<u8> {
        let file = self.writer.finish().unwrap();
        file.into_inner()
    }
}
