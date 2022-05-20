use namui::{
    animation::{KeyframeGraph, KeyframeValue, Layer},
    prelude::*,
    types::{PixelSize, Time},
};
use namui_prebuilt::{table::vertical, typography::center_text, *};

pub(crate) struct PropertyWindow {}

pub(crate) struct Props<'a> {
    pub layer: Option<&'a Layer>,
}

impl PropertyWindow {
    pub(crate) fn new() -> Self {
        Self {}
    }
    pub(crate) fn update(&mut self, event: &dyn std::any::Any) {}
}

impl table::CellRender<Props<'_>> for PropertyWindow {
    fn render(&self, wh: Wh<f32>, props: Props<'_>) -> RenderingTree {
        // let properties = match props.layer {
        //     Some(_) => [
        //         Property {
        //             name: "X".to_string(),
        //             unit: "px".to_string(),
        //         },
        //         Property {
        //             name: "Y".to_string(),
        //             unit: "px".to_string(),
        //         },
        //         Property {
        //             name: "Width".to_string(),
        //             unit: "px".to_string(),
        //         },
        //         Property {
        //             name: "Height".to_string(),
        //             unit: "px".to_string(),
        //         },
        //         Property {
        //             name: "Rotation".to_string(),
        //             unit: "°".to_string(),
        //         },
        //         Property {
        //             name: "Sprite".to_string(),
        //             unit: "/ 10".to_string(),
        //         },
        //         Property {
        //             name: "Visibility".to_string(),
        //             unit: "".to_string(),
        //         },
        //         Property {
        //             name: "Left Line".to_string(),
        //             unit: "".to_string(),
        //         },
        //         Property {
        //             name: "Right Line".to_string(),
        //             unit: "".to_string(),
        //         },
        //     ],
        //     None => [],
        // };
        render![
            simple_rect(wh, Color::BLACK, 1.0, Color::WHITE),
            vertical(chains![
                [fixed!(20.0, |wh| render_header(wh))],
                [
                    ratio!(1.0, |wh| render_property_row(
                        wh,
                        "X",
                        &props.layer.unwrap().image.x
                    )),
                    ratio!(1.0, |wh| render_property_row(
                        wh,
                        "Y",
                        &props.layer.unwrap().image.y
                    )),
                    ratio!(1.0, |wh| render_property_row(
                        wh,
                        "Width",
                        &props.layer.unwrap().image.width
                    )),
                    ratio!(1.0, |wh| render_property_row(
                        wh,
                        "Height",
                        &props.layer.unwrap().image.height
                    )),
                    ratio!(1.0, |wh| render_property_row(
                        wh,
                        "Rotation",
                        &props.layer.unwrap().image.x
                    )),
                    ratio!(1.0, |wh| render_property_row(
                        wh,
                        "Visibility",
                        &props.layer.unwrap().image.x
                    )),
                ],
                // properties
                //     .iter()
                //     .map(|property| ratio!(1.0, |wh| render_property_row(wh, property)))
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

impl PropertyEditCell for KeyframeGraph<PixelSize> {
    fn render_property_edit_cell(&self, wh: Wh<f32>) -> RenderingTree {
        render![
            simple_rect(wh, Color::BLACK, 1.0, Color::WHITE),
            typography::body::right(
                wh,
                format!(
                    "{:?} {}",
                    self.get_value(&Time::from_ms(0.0)),
                    PixelSize::unit()
                ),
                Color::BLACK
            ),
        ]
    }
}

fn render_graph_visible_toggle_cell(wh: Wh<f32>) -> RenderingTree {
    render![
        simple_rect(wh, Color::BLACK, 1.0, Color::WHITE),
        center_text(wh, "✅", Color::BLACK),
    ]
}
