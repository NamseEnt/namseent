use namui::{animation::Layer, prelude::*};
use namui_prebuilt::{table::vertical, *};

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
        let properties = [
            Property {
                name: "X".to_string(),
            },
            Property {
                name: "Y".to_string(),
            },
            Property {
                name: "Width".to_string(),
            },
            Property {
                name: "Height".to_string(),
            },
            Property {
                name: "Rotation".to_string(),
            },
            Property {
                name: "Sprite".to_string(),
            },
            Property {
                name: "Visibility".to_string(),
            },
            Property {
                name: "Left Line".to_string(),
            },
            Property {
                name: "Right Line".to_string(),
            },
        ];
        render![
            simple_rect(wh, Color::BLACK, 1.0, Color::WHITE),
            vertical(chains![
                [fixed!(20.0, |wh| render_header(wh))],
                properties
                    .iter()
                    .map(|property| ratio!(1.0, |wh| render_property_row(wh, property)))
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

struct Property {
    name: String,
}

fn render_property_row(wh: Wh<f32>, property: &Property) -> RenderingTree {
    render![
        simple_rect(wh, Color::BLACK, 1.0, Color::WHITE),
        horizontal![
            calculative!(|parent_wh| parent_wh.height, |wh| {
                render_graph_visible_toggle_cell(wh)
            }),
            ratio!(3.0, |wh| {
                render![
                    simple_rect(wh, Color::BLACK, 1.0, Color::WHITE),
                    center_text(wh, &property.name, Color::BLACK),
                ]
            }),
            ratio!(3.0, |wh| { simple_rect(wh, Color::BLACK, 1.0, Color::RED) }),
            ratio!(1.0, |wh| {
                simple_rect(wh, Color::BLACK, 1.0, Color::WHITE)
            }),
        ](wh),
    ]
}

fn render_graph_visible_toggle_cell(wh: Wh<f32>) -> RenderingTree {
    render![
        simple_rect(wh, Color::BLACK, 1.0, Color::WHITE),
        center_text(wh, "✅", Color::BLACK),
    ]
}
