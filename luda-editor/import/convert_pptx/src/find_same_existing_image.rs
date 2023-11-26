use crate::get_image_hash::get_image_hash;
use include_dir::include_dir;
use namui_type::Uuid;
use std::{
    collections::HashMap,
    fs::File,
    io::{self, Write},
    path::{Path, PathBuf},
    sync::{Mutex, OnceLock},
};
use zip::ZipArchive;

static EXISTING_IMAGES: OnceLock<Mutex<ExistingImages>> = OnceLock::new();

pub fn find_same_existing_image(ppt_image_path: &Path, zipper: &mut ZipArchive<File>) -> Result {
    let mut existing_images = EXISTING_IMAGES
        .get_or_init(ExistingImages::new)
        .lock()
        .unwrap();
    if let Some(cached) = existing_images.cache.get(ppt_image_path) {
        return cached.clone();
    }

    let mut ppt_image = Vec::new();
    io::copy(
        &mut zipper.by_name(ppt_image_path.to_str().unwrap()).unwrap(),
        &mut ppt_image,
    )
    .unwrap();
    let image_hash = get_image_hash(ppt_image.as_ref());
    let result =
        if let Some(existing_image_name) = existing_images.image_hash_name_map.get(&image_hash) {
            Result::ExistingImage(existing_image_name.clone())
        } else {
            let ext_name = ppt_image_path.extension().unwrap().to_str().unwrap();
            let new_image_name = format!("{}.{ext_name}", existing_images.next_name_num);
            File::create(format!("../src/images/{new_image_name}"))
                .unwrap()
                .write_all(&ppt_image)
                .unwrap();
            existing_images
                .image_hash_name_map
                .insert(image_hash, new_image_name.clone());
            let result = Result::NewImage(new_image_name);
            existing_images.next_name_num += 1;
            result
        };
    existing_images
        .cache
        .insert(ppt_image_path.to_path_buf(), result.clone());
    result
}

#[derive(Clone)]
pub enum Result {
    ExistingImage(String),
    NewImage(String),
}
impl Result {
    pub fn image_name(&self) -> &str {
        match self {
            Self::ExistingImage(name) => name,
            Self::NewImage(name) => name,
        }
    }
}

struct ExistingImages {
    image_hash_name_map: HashMap<Uuid, String>,
    next_name_num: isize,
    cache: HashMap<PathBuf, Result>,
}
impl ExistingImages {
    fn new() -> Mutex<Self> {
        let mut image_hash_name_map = HashMap::new();
        let mut last_name_num = 0;

        for image in include_dir!("../src/images").files() {
            let name_num = image
                .path()
                .file_stem()
                .unwrap()
                .to_str()
                .unwrap()
                .parse()
                .unwrap();
            if name_num >= last_name_num {
                last_name_num = name_num;
            }

            let image_hash = get_image_hash(image.contents());
            let name = image
                .path()
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string();

            image_hash_name_map.insert(image_hash, name);
        }

        Mutex::new(Self {
            image_hash_name_map,
            next_name_num: last_name_num + 1,
            cache: HashMap::new(),
        })
    }
}
