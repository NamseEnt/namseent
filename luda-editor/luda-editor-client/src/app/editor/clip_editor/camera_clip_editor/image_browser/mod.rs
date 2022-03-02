use namui::prelude::*;
use std::collections::BTreeSet;
mod browser_item;
use browser_item::*;
mod back_button;
mod empty_button;
mod scroll;
use scroll::*;
mod types;
pub use types::*;

#[derive(Debug)]
pub struct ImageBrowser {
    directory: ImageBrowserDirectory,
    selected_item: Option<ImageBrowserItem>,
    scroll: Scroll,
    thumbnail_url_prefix: String,
}
pub struct ImageBrowserProps<'a> {
    pub width: f32,
    pub height: f32,
    pub files: &'a BTreeSet<ImageBrowserFile>,
}

impl ImageBrowser {
    pub fn new(
        directory: ImageBrowserDirectory,
        selected_item: Option<ImageBrowserItem>,
        thumbnail_url_prefix: &str,
    ) -> Self {
        Self {
            directory,
            selected_item,
            scroll: Scroll::new(),
            thumbnail_url_prefix: thumbnail_url_prefix.to_string(),
        }
    }
    pub fn update(&mut self, event: &dyn std::any::Any) {
        if let Some(event) = event.downcast_ref::<ImageBrowserEvent>() {
            match event {
                ImageBrowserEvent::Select(selected_item) => match selected_item {
                    ImageBrowserItem::Back => {
                        let last_directory = self.directory.clone();
                        self.directory.navigate_to_parent();
                        self.selected_item = if self.directory.is_root() {
                            None
                        } else {
                            Some(ImageBrowserItem::Directory(last_directory))
                        };
                    }
                    ImageBrowserItem::Empty => {
                        self.selected_item = Some(ImageBrowserItem::Empty);
                    }
                    ImageBrowserItem::Directory(directory) => {
                        self.directory = directory.clone();
                        self.selected_item = None;
                    }
                    ImageBrowserItem::File(file) => {
                        self.selected_item = Some(ImageBrowserItem::File(file.clone()));
                    }
                },
            }
        };
        self.scroll.update(event);
    }

    pub fn render(&self, props: &ImageBrowserProps) -> RenderingTree {
        let current_directory_label_layout = XywhRect {
            x: 20.0,
            y: 20.0,
            width: 160.0,
            height: 40.0,
        };
        let is_root = self.directory.is_root();
        let item_margin = 10.0;
        let item_width = props.width / 2.0 - item_margin;
        let item_size = namui::Wh {
            width: item_width,
            height: item_width,
        };

        let thumbnail_rect = namui::XywhRect {
            x: 10.0,
            y: 5.0,
            width: item_size.width - 20.0,
            height: item_size.height - 20.0,
        };

        let get_browser_item_y =
            |index: usize| item_margin + (index / 2) as f32 * (item_size.height + item_margin);

        let mut browser_items = vec![];
        if !is_root {
            browser_items.push(self.render_back_button(item_size, thumbnail_rect));
        } else {
            browser_items.push(self.render_empty_button(item_size, thumbnail_rect));
        }
        browser_items.extend(
            self.get_directory_files_browser_item_props(item_size, thumbnail_rect, props.files)
                .iter()
                .map(|props| render_browser_item(props)),
        );
        let browser_items = browser_items
            .into_iter()
            .enumerate()
            .map(|(index, browser_item)| {
                namui::translate(
                    (index % 2) as f32 * (item_size.width + item_margin),
                    get_browser_item_y(index),
                    browser_item,
                )
            })
            .collect::<Vec<_>>();

        let browser_item_scroll_height =
            get_browser_item_y(browser_items.len() - 1) + item_size.height + item_margin;

        let scroll_bar_width = 10.0;

        namui::render![
            self.render_current_directory_label(&current_directory_label_layout),
            namui::translate(
                0.0,
                current_directory_label_layout.y,
                self.scroll.render(ScrollProps {
                    x: 0.0,
                    y: 0.0,
                    inner_width: props.width - scroll_bar_width,
                    inner_height: browser_item_scroll_height,
                    scroll_bar_width,
                    height: props.height
                        - (current_directory_label_layout.y
                            + current_directory_label_layout.height),
                    inner_rendering_tree: RenderingTree::Children(browser_items),
                }),
            )
        ]
    }

    fn render_current_directory_label(
        &self,
        current_directory_label_layout: &XywhRect<f32>,
    ) -> RenderingTree {
        namui::text(namui::TextParam {
            text: self.directory.to_string(),
            x: current_directory_label_layout.x,
            y: current_directory_label_layout.y,
            align: namui::TextAlign::Left,
            baseline: namui::TextBaseline::Bottom,
            font_type: namui::FontType {
                size: 16,
                serif: false,
                language: namui::Language::Ko,
                font_weight: namui::FontWeight::REGULAR,
            },
            style: namui::TextStyle {
                color: namui::Color::BLACK,
                ..Default::default()
            },
        })
    }

    fn get_directory_files_browser_item_props(
        &self,
        item_size: Wh<f32>,
        thumbnail_rect: XywhRect<f32>,
        files: &BTreeSet<ImageBrowserFile>,
    ) -> Vec<BrowserItemProps> {
        let under_directory_files = files
            .iter()
            .filter(|file| file.is_recursively_under_directory(&self.directory))
            .collect::<Vec<_>>();

        let just_under_directories = under_directory_files
            .iter()
            .filter_map(|file| {
                let directory = file.get_directory();
                let (diff, _) = directory.get_diff(&self.directory);
                let first_diff = diff.split('/').next();
                first_diff
                    .and_then(|first_diff| {
                        if first_diff.len() > 0 {
                            Some(first_diff.to_string())
                        } else {
                            None
                        }
                    })
                    .map(|first_diff| {
                        let mut directory = self.directory.clone();
                        directory.navigate_to_child(&first_diff);
                        ImageBrowserItem::Directory(directory)
                    })
            })
            .collect::<BTreeSet<_>>();

        let just_under_files = under_directory_files
            .iter()
            .filter(|file| file.is_just_under_directory(&self.directory))
            .map(|file| ImageBrowserItem::File((*file).clone()));

        just_under_directories
            .into_iter()
            .chain(just_under_files)
            .map(|item| BrowserItemProps {
                name: item.get_display_name().to_string(),
                thumbnail_url: self.get_thumbnail_url(&item, files),
                is_selected: self
                    .selected_item
                    .as_ref()
                    .map_or_else(|| false, |selected_item| selected_item.eq(&item)),
                item,
                item_size,
                thumbnail_rect,
            })
            .collect()
    }

    fn get_thumbnail_url(
        &self,
        item: &ImageBrowserItem,
        files: &BTreeSet<ImageBrowserFile>,
    ) -> Option<String> {
        match item {
            ImageBrowserItem::Back => unreachable!(),
            ImageBrowserItem::Empty => unreachable!(),
            ImageBrowserItem::Directory(directory) => files
                .iter()
                .find(|file| file.is_recursively_under_directory(directory))
                .and_then(|file| Some(file.get_url())),
            ImageBrowserItem::File(file) => Some(file.get_url()),
        }
        .map(|url| format!("{}{}", self.thumbnail_url_prefix, url))
    }

    pub(crate) fn select(&mut self, item: Option<ImageBrowserItem>) {
        if self.selected_item == item {
            return;
        }

        self.directory = match item.as_ref().unwrap() {
            ImageBrowserItem::Empty => ImageBrowserDirectory::root(),
            ImageBrowserItem::File(file) => file.get_directory(),
            _ => unreachable!(),
        };
        self.selected_item = item;
    }
}
