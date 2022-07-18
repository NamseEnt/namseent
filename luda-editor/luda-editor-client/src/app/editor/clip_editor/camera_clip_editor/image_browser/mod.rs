mod back_button;
mod browser_item;
mod empty_button;
mod scroll;
mod types;
use crate::app::storage::GithubStorage;
use browser_item::*;
use namui::prelude::*;
use scroll::*;
use std::{collections::BTreeSet, sync::Arc};
pub use types::*;

#[derive(Debug)]
pub struct ImageBrowser {
    id: String,
    directory: ImageBrowserDirectory,
    selected_item: Option<ImageBrowserItem>,
    scroll: Scroll,
    storage: Arc<dyn GithubStorage>,
    image_type: ImageType,
}
pub struct ImageBrowserProps<'a> {
    pub width: Px,
    pub height: Px,
    pub files: &'a BTreeSet<ImageBrowserFile>,
}

impl ImageBrowser {
    pub fn new(
        id: &str,
        directory: ImageBrowserDirectory,
        selected_item: Option<ImageBrowserItem>,
        storage: Arc<dyn GithubStorage>,
        image_type: ImageType,
    ) -> Self {
        Self {
            id: id.to_string(),
            directory,
            selected_item,
            scroll: Scroll::new(),
            storage,
            image_type,
        }
    }
    pub fn update(&mut self, event: &dyn std::any::Any) {
        if let Some(event) = event.downcast_ref::<ImageBrowserEvent>() {
            match event {
                ImageBrowserEvent::Select { browser_id, item } => {
                    if self.id.eq(browser_id) {
                        match item {
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
                        }
                    }
                }
            }
        };
        self.scroll.update(event);
    }

    pub fn render(&self, props: &ImageBrowserProps) -> RenderingTree {
        let current_directory_label_layout = Rect::Xywh {
            x: px(20.0),
            y: px(20.0),
            width: px(160.0),
            height: px(40.0),
        };
        let is_root = self.directory.is_root();
        let item_margin = px(10.0);
        let item_width = props.width / 2.0 - item_margin;
        let item_size = namui::Wh {
            width: item_width,
            height: item_width,
        };

        let thumbnail_rect = namui::Rect::Xywh {
            x: px(10.0),
            y: px(5.0),
            width: item_size.width - px(20.0),
            height: item_size.height - px(20.0),
        };

        let get_browser_item_y =
            |index: usize| item_margin + (index / 2) as f32 * (item_size.height + item_margin);

        let mut browser_items = vec![];
        if !is_root {
            browser_items.push(self.render_back_button(&self.id, item_size, thumbnail_rect));
        } else {
            browser_items.push(self.render_empty_button(&self.id, item_size, thumbnail_rect));
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
                    (index % 2) * (item_size.width + item_margin),
                    get_browser_item_y(index),
                    browser_item,
                )
            })
            .collect::<Vec<_>>();

        let browser_item_scroll_height =
            get_browser_item_y(browser_items.len() - 1) + item_size.height + item_margin;

        let scroll_bar_width = px(10.0);

        namui::render([
            self.render_current_directory_label(current_directory_label_layout),
            namui::translate(
                px(0.0),
                current_directory_label_layout.y(),
                self.scroll.render(ScrollProps {
                    x: px(0.0),
                    y: px(0.0),
                    inner_width: props.width - scroll_bar_width,
                    inner_height: browser_item_scroll_height,
                    scroll_bar_width,
                    height: props.height
                        - (current_directory_label_layout.y()
                            + current_directory_label_layout.height()),
                    inner_rendering_tree: RenderingTree::Children(browser_items),
                }),
            ),
        ])
    }

    fn render_current_directory_label(
        &self,
        current_directory_label_layout: Rect<Px>,
    ) -> RenderingTree {
        namui::text(namui::TextParam {
            text: self.directory.to_string(),
            x: current_directory_label_layout.x(),
            y: current_directory_label_layout.y(),
            align: namui::TextAlign::Left,
            baseline: namui::TextBaseline::Bottom,
            font_type: namui::FontType {
                size: int_px(16),
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
        item_size: Wh<Px>,
        thumbnail_rect: Rect<Px>,
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
                browser_id: self.id.clone(),
            })
            .collect()
    }

    fn get_thumbnail_url(
        &self,
        item: &ImageBrowserItem,
        files: &BTreeSet<ImageBrowserFile>,
    ) -> Option<Url> {
        match item {
            ImageBrowserItem::Back => unreachable!(),
            ImageBrowserItem::Empty => unreachable!(),
            ImageBrowserItem::Directory(directory) => files
                .iter()
                .find(|file| file.is_recursively_under_directory(directory))
                .and_then(|file| Some(file.get_path())),
            ImageBrowserItem::File(file) => Some(file.get_path()),
        }
        .map(|path| match self.image_type {
            ImageType::Character => self.storage.get_character_image_url(path.as_str()).unwrap(),
            ImageType::Background => self
                .storage
                .get_background_image_url(path.as_str())
                .unwrap(),
        })
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

    pub(crate) fn get_id(&self) -> String {
        self.id.clone()
    }
}
