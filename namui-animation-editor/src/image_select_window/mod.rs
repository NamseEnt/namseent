use crate::types::{Act, AnimationHistory};
use namui::{animation::Animation, file::types::Dirent, prelude::*};
use namui_prebuilt::{
    table::{horizontal, ratio, vertical},
    *,
};

pub struct ImageSelectWindow {
    animation_history: AnimationHistory,
    list_view: list_view::ListView,
}

pub struct Props {
    pub wh: Wh<Px>,
    pub selected_layer_image_url: Option<Url>,
    pub selected_layer_id: Option<Uuid>,
}

#[derive(Debug, Clone)]
enum Event {
    ImageSelected {
        url: Url,
        selected_layer_id: namui::Uuid,
    },
}

impl ImageSelectWindow {
    pub fn new(animation_history: AnimationHistory) -> Self {
        Self {
            list_view: list_view::ListView::new(),
            animation_history,
        }
    }
    pub fn update(&mut self, event: &namui::Event) {
        event.is::<Event>(|event| match event {
            Event::ImageSelected {
                url,
                selected_layer_id,
            } => {
                struct SelectImageAction {
                    url: Url,
                    layer_id: namui::Uuid,
                }
                impl Act<Animation> for SelectImageAction {
                    fn act(
                        &self,
                        state: &Animation,
                    ) -> Result<Animation, Box<dyn std::error::Error>> {
                        let mut animation = state.clone();

                        if let Some(layer) = animation
                            .layers
                            .iter_mut()
                            .find(|layer| layer.id.eq(&self.layer_id))
                        {
                            layer.image.image_source_url = Some(self.url.clone());
                            Ok(animation)
                        } else {
                            Err("layer not found".into())
                        }
                    }
                }

                if let Some(action_ticket) =
                    self.animation_history.try_set_action(SelectImageAction {
                        url: url.clone(),
                        layer_id: selected_layer_id.clone(),
                    })
                {
                    self.animation_history.act(action_ticket).unwrap();
                }
            }
        });
        self.list_view.update(event);
    }
    pub fn render(&self, props: Props) -> RenderingTree {
        let border = simple_rect(props.wh, Color::BLACK, 1.px(), Color::WHITE);

        let content = if props.selected_layer_id.is_none() {
            RenderingTree::Empty
        } else {
            let selected_layer_id = props.selected_layer_id.unwrap();
            let selected_layer_image_url = props.selected_layer_image_url.clone();
            let dir = namui::file::bundle::read_dir("img").unwrap();
            const COLUMN_COUNT: usize = 2;
            let grouped_entries =
                dir.into_iter()
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
                xy: Xy::single(px(0.0)),
                height: props.wh.height,
                item_wh: Wh {
                    width: props.wh.width,
                    height: props.wh.width / 2.0,
                },
                scroll_bar_width: px(1.0),
                items: grouped_entries,
                item_render: move |wh, entries| {
                    let mut column_entries: Vec<Option<Dirent>> =
                        entries.into_iter().map(|vec| Some(vec)).collect();

                    while column_entries.len() < COLUMN_COUNT {
                        column_entries.push(None);
                    }

                    let selected_layer_id = selected_layer_id.clone();
                    let selected_layer_image_url = selected_layer_image_url.clone();

                    horizontal(column_entries.into_iter().map(move |entry| {
                        let selected_layer_id = selected_layer_id.clone();
                        let selected_layer_image_url = selected_layer_image_url.clone();

                        ratio(1.0, move |wh| match entry {
                            Some(entry) => {
                                let selected_layer_id = selected_layer_id.clone();
                                let selected_layer_image_url = selected_layer_image_url.clone();

                                let is_selected =
                                    selected_layer_image_url == Some(entry.url().clone());
                                render_entry(wh, &entry, is_selected, selected_layer_id)
                            }
                            None => RenderingTree::Empty,
                        })
                    }))(wh)
                },
            })
        };
        render([border, content])
    }
}

fn render_entry(
    wh: Wh<Px>,
    entry: &Dirent,
    is_selected: bool,
    selected_layer_id: namui::Uuid,
) -> RenderingTree {
    let selection_highlight_box = match is_selected {
        true => simple_rect(wh, Color::RED, px(3.0), Color::TRANSPARENT),
        false => RenderingTree::Empty,
    };
    namui::render([
        vertical([
            ratio(0.8, |wh| render_thumbnail(wh, entry)),
            ratio(0.2, |wh| {
                typography::center_text_full_height(
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
            let selected_layer_id = selected_layer_id.clone();
            builder.on_mouse_down_in(move |_| {
                namui::event::send(Event::ImageSelected {
                    url: url.clone(),
                    selected_layer_id: selected_layer_id.clone(),
                })
            });
        }),
        selection_highlight_box,
    ])
}

fn render_thumbnail(wh: Wh<Px>, entry: &Dirent) -> RenderingTree {
    if entry.is_dir() {
        return RenderingTree::Empty;
    }

    let url = entry.url();

    namui::image(namui::ImageParam {
        rect: Rect::from_xy_wh(Xy::single(px(0.0)), wh),
        source: ImageSource::Url(url.clone()),
        style: ImageStyle {
            fit: ImageFit::Contain,
            paint_builder: None,
        },
    })
}
