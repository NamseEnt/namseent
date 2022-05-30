use namui::{
    animation::{KeyframeGraph, Layer},
    prelude::*,
    types::{PixelSize, Time},
};
use namui_prebuilt::{table::vertical, typography::center_text, *};
use std::sync::{Arc, RwLock};

pub(crate) struct PropertyWindow {
    animation: Arc<RwLock<animation::Animation>>,
    layer_id: String,
    input_text: Option<String>,
    x_text_input: namui::TextInput,
    y_text_input: namui::TextInput,
    width_text_input: namui::TextInput,
    height_text_input: namui::TextInput,
}

pub(crate) struct Props {}

impl PropertyWindow {
    pub(crate) fn new(animation: Arc<RwLock<animation::Animation>>, layer_id: String) -> Self {
        Self {
            animation,
            layer_id,
            input_text: None,
            x_text_input: namui::TextInput::new(),
            y_text_input: namui::TextInput::new(),
            width_text_input: namui::TextInput::new(),
            height_text_input: namui::TextInput::new(),
        }
    }
    pub(crate) fn update(&mut self, event: &dyn std::any::Any) {
        struct F32Input<'a> {
            text_input: &'a mut TextInput,
            get_graph_mut: for<'b> fn(&'b mut Layer) -> &'b mut KeyframeGraph<PixelSize>,
            get_graph: for<'b> fn(&'b Layer) -> &'b KeyframeGraph<PixelSize>,
        }
        let mut f32_inputs = [
            F32Input {
                text_input: &mut self.x_text_input,
                get_graph_mut: |layer| &mut layer.image.x,
                get_graph: |layer| &layer.image.x,
            },
            F32Input {
                text_input: &mut self.y_text_input,
                get_graph_mut: |layer| &mut layer.image.y,
                get_graph: |layer| &layer.image.y,
            },
            F32Input {
                text_input: &mut self.width_text_input,
                get_graph_mut: |layer| &mut layer.image.width,
                get_graph: |layer| &layer.image.width,
            },
            F32Input {
                text_input: &mut self.height_text_input,
                get_graph_mut: |layer| &mut layer.image.height,
                get_graph: |layer| &layer.image.height,
            },
        ];

        f32_inputs.iter_mut().for_each(|input| {
            input.text_input.update(event);
        });

        if let Some(event) = event.downcast_ref::<text_input::Event>() {
            match event {
                text_input::Event::TextUpdated(text_updated) => {
                    if f32_inputs
                        .iter()
                        .any(|input| input.text_input.get_id() == &text_updated.id)
                    {
                        self.input_text = Some(text_updated.text.clone());
                    }
                }
                text_input::Event::Focus(focus) => {
                    let animation = self.animation.read().unwrap();
                    let layer = animation
                        .layers
                        .iter()
                        .find(|layer| layer.id.eq(&self.layer_id));
                    if layer.is_none() {
                        namui::event::send(crate::Event::Error(format!(
                            "Could not find layer with id {}",
                            self.layer_id
                        )));
                    };
                    let layer = layer.unwrap();
                    let time = Time::from_ms(0.0);

                    f32_inputs
                        .iter()
                        .find(|input| input.text_input.get_id().eq(&focus.id))
                        .map(|input| {
                            if self.input_text.is_none() {
                                let graph = (input.get_graph)(layer);
                                self.input_text =
                                    Some(graph.get_value(&time).map_or("".to_string(), |value| {
                                        f32::from(value).to_string()
                                    }));
                            }
                        });
                }
                text_input::Event::Blur(blur) => {
                    let animation = self.animation.read().unwrap();
                    let layer = animation
                        .layers
                        .iter()
                        .find(|layer| layer.id.eq(&self.layer_id));
                    if layer.is_none() {
                        namui::event::send(crate::Event::Error(format!(
                            "Could not find layer with id {}",
                            self.layer_id
                        )));
                    };
                    let layer = layer.unwrap();

                    let time = Time::from_ms(0.0); // TODO

                    f32_inputs
                        .iter_mut()
                        .find(|input| input.text_input.get_id().eq(&blur.id))
                        .map(|input| {
                            let event = {
                                let mut next_layer = layer.clone();
                                let graph = (input.get_graph_mut)(&mut next_layer);
                                let input_text = self.input_text.as_ref().unwrap();
                                if input_text.is_empty() {
                                    graph.delete(time);
                                    crate::Event::UpdateLayer(Arc::new(next_layer))
                                } else {
                                    if let Ok(value) = input_text.parse::<f32>() {
                                        graph.put(
                                            namui::animation::KeyframePoint {
                                                time,
                                                value: value.into(),
                                            },
                                            animation::KeyframeLine::Linear,
                                        );
                                        crate::Event::UpdateLayer(Arc::new(next_layer))
                                    } else {
                                        crate::Event::Error(format!(
                                            "{} is not a valid number",
                                            input_text
                                        ))
                                    }
                                }
                            };

                            namui::event::send(event);
                            self.input_text = None;
                        });
                }
                _ => {}
            }
        }
    }
}

impl table::CellRender<Props> for PropertyWindow {
    fn render(&self, wh: Wh<f32>, props: Props) -> RenderingTree {
        let animation = self.animation.read().unwrap();
        let layer = animation
            .layers
            .iter()
            .find(|layer| layer.id.eq(&self.layer_id));
        if layer.is_none() {
            return RenderingTree::Empty;
        }
        let layer = layer.unwrap();
        render![
            simple_rect(wh, Color::BLACK, 1.0, Color::WHITE),
            vertical(chains![
                [fixed!(20.0, |wh| render_header(wh))],
                [
                    ratio!(1.0, |wh| render_property_row(
                        wh,
                        "X",
                        &(self, &layer.image.x, &self.x_text_input),
                    )),
                    ratio!(1.0, |wh| render_property_row(
                        wh,
                        "Y",
                        &(self, &layer.image.y, &self.y_text_input),
                    )),
                    ratio!(1.0, |wh| render_property_row(
                        wh,
                        "Width",
                        &(self, &layer.image.width, &self.width_text_input),
                    )),
                    ratio!(1.0, |wh| render_property_row(
                        wh,
                        "Height",
                        &(self, &layer.image.height, &self.height_text_input),
                    )),
                    ratio!(1.0, |wh|
                        // render_property_row(
                        //     wh,
                        //     "Rotation",
                        //     &self.layer.image.x
                        // )
                        RenderingTree::Empty),
                    ratio!(1.0, |wh|
                        // render_property_row(
                        //     wh,
                        //     "Visibility",
                        //     &self.layer.image.x
                        // )
                        RenderingTree::Empty),
                ],
            ])(wh)
        ]
    }
}

fn render_header(wh: Wh<f32>) -> RenderingTree {
    // TODO : Add Eyes
    render![
        simple_rect(wh, Color::BLACK, 1.0, Color::BLACK),
        namui::text(TextParam {
            text: "Property".to_string(),
            x: wh.width / 2.0,
            y: wh.height / 2.0,
            align: TextAlign::Center,
            baseline: TextBaseline::Middle,
            font_type: FontType {
                font_weight: FontWeight::REGULAR,
                language: Language::Ko,
                serif: false,
                size: crate::adjust_font_size(wh.height),
            },
            style: TextStyle {
                color: Color::WHITE,
                ..Default::default()
            }
        })
    ]
}

fn render_property_row<T: PropertyEditCell>(
    wh: Wh<f32>,
    name: &str,
    property: &T,
) -> RenderingTree {
    render![
        simple_rect(wh, Color::BLACK, 1.0, Color::WHITE),
        horizontal![
            calculative!(|parent_wh| parent_wh.height, |wh| {
                render_graph_visible_toggle_cell(wh)
            }),
            ratio!(3.0, |wh| {
                render![
                    simple_rect(wh, Color::BLACK, 1.0, Color::WHITE),
                    typography::body::center(wh, &name, Color::BLACK),
                ]
            }),
            ratio!(3.0, |wh| { property.render_property_edit_cell(wh) }),
        ](wh),
    ]
}

trait PropertyEditCell {
    fn render_property_edit_cell(&self, wh: Wh<f32>) -> RenderingTree;
}

impl PropertyEditCell
    for (
        &'_ PropertyWindow,
        &'_ KeyframeGraph<PixelSize>,
        &'_ namui::TextInput,
    )
{
    fn render_property_edit_cell(&self, wh: Wh<f32>) -> RenderingTree {
        let (window, pixel_size, text_input) = &self;

        let text = if window.input_text.is_some() && text_input.is_focused() {
            window.input_text.clone().unwrap()
        } else {
            let value = pixel_size.get_value(&Time::from_ms(0.0));
            value.map_or("".to_string(), |v| f32::from(v).to_string())
        };

        render![
            simple_rect(wh, Color::BLACK, 1.0, Color::WHITE),
            text_input.render(text_input::Props {
                rect_param: RectParam {
                    x: 0.0,
                    y: 0.0,
                    width: wh.width,
                    height: wh.height,
                    style: RectStyle {
                        stroke: Some(RectStroke {
                            border_position: BorderPosition::Inside,
                            color: Color::BLACK,
                            width: 1.0,
                        }),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                text_param: TextParam {
                    text,
                    x: wh.width,
                    y: wh.height / 2.0,
                    align: TextAlign::Right,
                    baseline: TextBaseline::Middle,
                    font_type: FontType {
                        font_weight: FontWeight::REGULAR,
                        language: Language::Ko,
                        serif: false,
                        size: 12,
                    },
                    style: TextStyle {
                        color: Color::BLACK,
                        ..Default::default()
                    },
                }
            }),
            // typography::body::right(
            //     wh,
            //     format!(
            //         "{:?} {}",
            //         pixel_size.get_value(&Time::from_ms(0.0)),
            //         PixelSize::unit()
            //     ),
            //     Color::BLACK
            // ),
        ]
    }
}

fn render_graph_visible_toggle_cell(wh: Wh<f32>) -> RenderingTree {
    render![
        simple_rect(wh, Color::BLACK, 1.0, Color::WHITE),
        center_text(wh, "✅", Color::BLACK),
    ]
}
