use super::menu;
use namui::prelude::*;

pub struct ImageLoader {
    image_loader_state: ImageLoaderState,
}

enum ImageLoaderState {
    Idle,
    Loading {
        total_image_count: usize,
        loaded_image_count: usize,
    },
    Loaded,
    Failed(Box<dyn std::error::Error>),
}

impl ImageLoader {
    pub fn new() -> Self {
        Self {
            image_loader_state: ImageLoaderState::Idle,
        }
    }

    pub fn update(&mut self, event: &namui::Event) {
        event
            .is::<InternalEvent>(|event| match event {
                InternalEvent::ImageLoaded => self.on_image_loaded(),
            })
            .is::<menu::Event>(|event| match event {
                menu::Event::StartNewButtonClicked => self.start_load_all_images(),
            });
    }

    fn start_load_all_images(&mut self) {
        const CONCURRENT: usize = 4;
        let ImageLoaderState::Idle = self.image_loader_state  else{
            return;
        };

        let Ok(image_urls) = get_image_urls() else {
            namui::log!("failed to get image urls");
            return;
        };
        self.image_loader_state = ImageLoaderState::Loading {
            total_image_count: image_urls.len(),
            loaded_image_count: 0,
        };
        load_images_concurrently(image_urls, CONCURRENT)
    }

    fn on_image_loaded(&mut self) {
        let ImageLoaderState::Loading {
            total_image_count,
            loaded_image_count,
        } = &mut self.image_loader_state else {return;};

        *loaded_image_count += 1;

        let all_image_loaded = total_image_count == loaded_image_count;
        if all_image_loaded {
            self.image_loader_state = ImageLoaderState::Loaded;
        }
    }
}

enum InternalEvent {
    ImageLoaded,
}

fn get_image_urls() -> Result<Vec<Url>, Box<dyn std::error::Error>> {
    let mut directory_path_queue = vec!["image".to_string()];
    let mut image_urls = Vec::new();
    while let Some(directory_path) = directory_path_queue.pop() {
        let directory = namui::file::bundle::read_dir(directory_path.as_str())
            .map_err(|error| format!("{error:?}"))?;
        for entry in directory {
            let path = entry.path_string().to_string();
            match entry.kind() {
                namui::file::types::DirentKind::Directory => directory_path_queue.push(path),
                namui::file::types::DirentKind::File => {
                    if check_path_is_image_path(&path) {
                        image_urls.push(entry.url().clone())
                    }
                }
            }
        }
    }
    Ok(image_urls)
}

fn check_path_is_image_path(path: &String) -> bool {
    const IMAGE_EXTENSION_NAMES: [&str; 2] = ["png", "jpg"];
    IMAGE_EXTENSION_NAMES
        .iter()
        .any(|extention_name| path.ends_with(extention_name))
}

fn load_images_concurrently(image_urls: Vec<Url>, concurrent: usize) {
    split_image_urls_into_task_size(image_urls, concurrent)
        .into_iter()
        .for_each(|image_urls| {
            spawn_local(async move {
                for image_url in image_urls {
                    namui::image::load_url(&image_url).await;
                    namui::event::send(InternalEvent::ImageLoaded);
                }
            })
        });
}

fn split_image_urls_into_task_size(image_urls: Vec<Url>, concurrent: usize) -> Vec<Vec<Url>> {
    let mut splited = (0..concurrent).map(|_| Vec::new()).collect::<Vec<_>>();
    image_urls
        .into_iter()
        .enumerate()
        .for_each(|(index, image_url)| {
            splited[index % concurrent].push(image_url);
        });
    splited
}
