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
    path: String,
}
impl ImageBrowserFile {
    pub fn new(path: String) -> Self {
        Self { path }
    }
    fn get_name(&self) -> String {
        Path::new(&self.path)
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string()
    }
    pub(crate) fn get_directory(&self) -> ImageBrowserDirectory {
        let path = Path::new(&self.path);
        let parent = path.parent().unwrap();
        let parent_path = parent.to_str().unwrap().to_string();
        ImageBrowserDirectory::new(parent_path)
    }
    pub(crate) fn is_recursively_under_directory(&self, directory: &ImageBrowserDirectory) -> bool {
        let directory_url_with_suffix_slash = if directory.path.ends_with("/") {
            directory.path.clone()
        } else {
            format!("{}/", directory.path)
        };
        self.path.starts_with(&directory_url_with_suffix_slash)
    }
    pub(crate) fn is_just_under_directory(&self, directory: &ImageBrowserDirectory) -> bool {
        let path = Path::new(&self.path);
        let parent = path.parent().unwrap();
        let parent_path = parent.to_str().unwrap().to_string();
        parent_path == directory.path
    }
    pub fn get_path(&self) -> String {
        self.path.clone()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ImageBrowserDirectory {
    path: String,
}
impl ImageBrowserDirectory {
    pub fn new(path: String) -> ImageBrowserDirectory {
        Self { path }
    }
    pub fn root() -> Self {
        Self {
            path: "/".to_string(),
        }
    }
    pub(crate) fn navigate_to_parent(&mut self) {
        let path = Path::new(&self.path);
        let parent = path.parent().unwrap();
        let parent_path = parent.to_str().unwrap().to_string();
        self.path = parent_path;
    }
    pub(crate) fn to_string(&self) -> String {
        self.path.clone()
    }
    pub(crate) fn is_root(&self) -> bool {
        self.path == "/"
    }
    pub(crate) fn top(&self) -> String {
        let path = Path::new(&self.path);
        path.file_name().unwrap().to_str().unwrap().to_string()
    }
    #[allow(dead_code)]
    pub(crate) fn is_just_under_directory(&self, directory: &ImageBrowserDirectory) -> bool {
        let path = Path::new(&self.path);
        let parent = path.parent().unwrap();
        let parent_path = parent.to_str().unwrap().to_string();
        parent_path == directory.path
    }
    #[allow(dead_code)]
    pub(crate) fn is_recursively_under_directory(&self, directory: &ImageBrowserDirectory) -> bool {
        self.path
            .starts_with(format!("{}/", directory.path).as_str())
    }
    pub(crate) fn get_diff(&self, directory: &ImageBrowserDirectory) -> (String, String) {
        if self.path == directory.path {
            return ("".to_string(), "".to_string());
        }
        let self_chunks = self.path.split("/").collect::<Vec<&str>>();
        let directory_chunks = directory.path.split("/").collect::<Vec<&str>>();

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
        if !self.path.ends_with("/") {
            self.path.push_str("/");
        }

        self.path.push_str(sub_url);
    }
}

pub enum ImageBrowserEvent {
    Select {
        browser_id: String,
        item: ImageBrowserItem,
    },
}

#[derive(Debug)]
pub enum ImageType {
    Character,
    Background,
}
