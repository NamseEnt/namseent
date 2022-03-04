use std::path::Path;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ImageBrowserItem {
    Back,
    Empty,
    Directory(ImageBrowserDirectory),
    File(ImageBrowserFile),
}

impl ImageBrowserItem {
    pub fn get_display_name(&self) -> String {
        match self {
            ImageBrowserItem::Back => "Back".to_string(),
            ImageBrowserItem::Empty => "Empty".to_string(),
            ImageBrowserItem::Directory(dir) => dir.top(),
            ImageBrowserItem::File(file) => file.get_name(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ImageBrowserFile {
    url: String,
}
impl ImageBrowserFile {
    pub fn new(url: String) -> Self {
        Self { url }
    }
    fn get_name(&self) -> String {
        Path::new(&self.url)
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string()
    }
    pub(crate) fn get_directory(&self) -> ImageBrowserDirectory {
        let path = Path::new(&self.url);
        let parent = path.parent().unwrap();
        let parent_path = parent.to_str().unwrap().to_string();
        ImageBrowserDirectory::new(parent_path)
    }
    pub(crate) fn is_recursively_under_directory(&self, directory: &ImageBrowserDirectory) -> bool {
        let directory_url_with_suffix_slash = if directory.url.ends_with("/") {
            directory.url.clone()
        } else {
            format!("{}/", directory.url)
        };
        self.url.starts_with(&directory_url_with_suffix_slash)
    }
    pub(crate) fn is_just_under_directory(&self, directory: &ImageBrowserDirectory) -> bool {
        let path = Path::new(&self.url);
        let parent = path.parent().unwrap();
        let parent_path = parent.to_str().unwrap().to_string();
        parent_path == directory.url
    }
    pub fn get_url(&self) -> String {
        self.url.clone()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ImageBrowserDirectory {
    url: String,
}
impl ImageBrowserDirectory {
    pub fn new(url: String) -> ImageBrowserDirectory {
        Self { url }
    }
    pub fn root() -> Self {
        Self {
            url: "/".to_string(),
        }
    }
    pub(crate) fn navigate_to_parent(&mut self) {
        let path = Path::new(&self.url);
        let parent = path.parent().unwrap();
        let parent_path = parent.to_str().unwrap().to_string();
        self.url = parent_path;
    }
    pub(crate) fn to_string(&self) -> String {
        self.url.clone()
    }
    pub(crate) fn is_root(&self) -> bool {
        self.url == "/"
    }
    pub(crate) fn top(&self) -> String {
        let path = Path::new(&self.url);
        path.file_name().unwrap().to_str().unwrap().to_string()
    }
    pub(crate) fn is_just_under_directory(&self, directory: &ImageBrowserDirectory) -> bool {
        let path = Path::new(&self.url);
        let parent = path.parent().unwrap();
        let parent_path = parent.to_str().unwrap().to_string();
        parent_path == directory.url
    }
    pub(crate) fn is_recursively_under_directory(&self, directory: &ImageBrowserDirectory) -> bool {
        self.url.starts_with(format!("{}/", directory.url).as_str())
    }
    pub(crate) fn get_diff(&self, directory: &ImageBrowserDirectory) -> (String, String) {
        if self.url == directory.url {
            return ("".to_string(), "".to_string());
        }
        let self_chunks = self.url.split("/").collect::<Vec<&str>>();
        let directory_chunks = directory.url.split("/").collect::<Vec<&str>>();

        /*
        expected result
          self: /a/b
          directory: /a/bc
          --> (b, bc)
        */
        let mut self_chunks_iter = self_chunks.into_iter().peekable();
        let mut directory_chunks_iter = directory_chunks.into_iter().peekable();
        loop {
            let self_chunk = self_chunks_iter.peek();
            let directory_chunk = directory_chunks_iter.peek();
            if self_chunk.is_none() || directory_chunk.is_none() || self_chunk != directory_chunk {
                break;
            }
            self_chunks_iter.next();
            directory_chunks_iter.next();
        }
        return (
            self_chunks_iter.collect::<Vec<_>>().join("/"),
            directory_chunks_iter.collect::<Vec<_>>().join("/"),
        );
    }

    pub(crate) fn navigate_to_child(&mut self, sub_url: &str) {
        if !self.url.ends_with("/") {
            self.url.push_str("/");
        }

        self.url.push_str(sub_url);
    }
}

pub enum ImageBrowserEvent {
    Select {
        browser_id: String,
        item: ImageBrowserItem,
    },
}
