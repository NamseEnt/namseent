use namui::{
    animation::Animate,
    prelude::*,
    system::file::types::Dirent,
    types::{Time, TimePerPixel},
};
use namui_prebuilt::{
    table::{horizontal, ratio, vertical},
    *,
};

pub(crate) struct ImageSelectWindow {
    list_view: list_view::ListView,
}

pub(crate) struct Props {
    pub selected_layer_image_url: Option<Url>,
}

#[derive(Debug, Clone)]
pub(crate) enum Event {
    ImageSelected(Url),
}

pub(crate) struct ImageSelectWindowContext {
    pub start_at: Time,
    pub time_per_pixel: TimePerPixel,
}

impl ImageSelectWindow {
    pub(crate) fn new() -> Self {
        Self {
            list_view: list_view::ListView::new(),
        }
    }
    pub(crate) fn update(&mut self, event: &dyn std::any::Any) {
        self.list_view.update(event);
    }
}

impl table::CellRender<Props> for ImageSelectWindow {
    fn render(&self, wh: Wh<f32>, props: Props) -> RenderingTree {
        let dir = namui::system::file::bundle::read_dir("img").unwrap();
        const COLUMN_COUNT: usize = 2;
        let grouped_entries = dir
            .into_iter()
            .fold(vec![], |mut acc: Vec<Vec<Dirent>>, entry| {
                match acc.last_mut() {
                    Some(vec) => {
                        if vec.len() == COLUMN_COUNT {
                            acc.push(vec![entry]);
                        } else {
                            vec.push(entry);
                        }
                    }
                    None => acc.push(vec![entry]),
                }
                acc
            });

        self.list_view.render(list_view::Props {
            x: 0.0,
            y: 0.0,
            height: wh.height,
            item_wh: Wh {
                width: wh.width,
                height: wh.width / 2.0,
            },
            scroll_bar_width: 1.0,
            items: grouped_entries,
            item_render: move |wh, entries| {
                horizontal((0..COLUMN_COUNT).map(|index| {
                    let entry = entries.get(index);
                    let selected_layer_image_url = props.selected_layer_image_url.clone();
                    ratio(1.0, move |wh| match entry {
                        Some(entry) => {
                            let is_selected = selected_layer_image_url == Some(entry.url().clone());
                            render_entry(wh, entry, is_selected)
                        }
                        None => RenderingTree::Empty,
                    })
                }))(wh)
            },
        })
    }
}

fn render_entry(wh: Wh<f32>, entry: &Dirent, is_selected: bool) -> RenderingTree {
    let selection_highlight_box = match is_selected {
        true => simple_rect(wh, Color::RED, 3.0, Color::TRANSPARENT),
        false => RenderingTree::Empty,
    };
    namui::render([
        vertical([
            ratio(0.8, |wh| render_thumbnail(wh, entry)),
            ratio(0.2, |wh| {
                typography::center_text(
                    wh,
                    entry
                        .path_buf()
                        .components()
                        .last()
                        .unwrap()
                        .as_os_str()
                        .to_string_lossy(),
                    Color::BLACK,
                )
            }),
        ])(wh)
        .attach_event(move |builder| {
            let url = entry.url().clone();
            builder.on_mouse_down(move |_| namui::event::send(Event::ImageSelected(url.clone())));
        }),
        selection_highlight_box,
    ])
}

fn render_thumbnail(wh: Wh<f32>, entry: &Dirent) -> RenderingTree {
    if entry.is_dir() {
        return RenderingTree::Empty;
    }

    let url = entry.url();

    namui::image(namui::ImageParam {
        xywh: XywhRect {
            x: 0.0,
            y: 0.0,
            width: wh.width,
            height: wh.height,
        },
        source: ImageSource::Url(url.clone()),
        style: ImageStyle {
            fit: ImageFit::Contain,
            paint_builder: None,
        },
    })
}
