use namui::prelude::*;
use std::f32::consts::PI;

const ICON_SIZE: Px = px(16.0);
const ICON_OFFSET: Px = px(8.0);

pub struct Tool {
    last_cursor_position: Xy<Px>,
    current_tool_type: ToolType,
    secondary_tool_type: Option<ToolType>,
}
impl Tool {
    pub fn new() -> Self {
        Self {
            last_cursor_position: Xy {
                x: px(0.0),
                y: px(0.0),
            },
            current_tool_type: ToolType::Hand,
            secondary_tool_type: None,
        }
    }

    pub fn render_cursor_icon(&self) -> RenderingTree {
        let tool_type = self.secondary_tool_type.unwrap_or(self.current_tool_type);
        absolute(
            self.last_cursor_position.x,
            self.last_cursor_position.y,
            match tool_type {
                ToolType::RectSelection => render_rect_selection_icon(),
                ToolType::PolySelection => render_poly_selection_icon(),
                ToolType::Hand => render_hand_icon(),
                ToolType::Zoom => render_zoom_icon(),
            },
        )
    }

    pub fn update(&mut self, event: &dyn std::any::Any) {
        if let Some(event) = event.downcast_ref::<NamuiEvent>() {
            match event {
                NamuiEvent::MouseMove(event) => self.last_cursor_position = event.xy.clone(),
                _ => (),
            };
        }
    }

    pub fn get_current_tool_type(&self) -> ToolType {
        let tool_type = self.secondary_tool_type.unwrap_or(self.current_tool_type);
        tool_type
    }

    pub fn change_tool_type(&mut self, to: ToolType) {
        self.current_tool_type = to;
    }

    pub fn set_secondary_tool_type(&mut self, to: ToolType) {
        self.secondary_tool_type = Some(to);
    }

    pub fn unset_secondary_tool_type(&mut self) {
        self.secondary_tool_type = None;
    }
}

#[derive(Clone, Copy)]
pub enum ToolType {
    RectSelection,
    PolySelection,
    Hand,
    Zoom,
}

fn render_rect_selection_icon() -> RenderingTree {
    namui::rect(RectParam {
        rect: Rect::Xywh {
            x: ICON_OFFSET,
            y: ICON_OFFSET,
            width: ICON_SIZE,
            height: ICON_SIZE,
        },
        style: RectStyle {
            stroke: Some(RectStroke {
                color: Color::grayscale_f01(0.3),
                width: px(2.0),
                border_position: namui::BorderPosition::Inside,
            }),
            fill: None,
            round: None,
        },
    })
}

fn render_poly_selection_icon() -> RenderingTree {
    let poly_selection_path = get_poly_selection_path()
        .scale(ICON_SIZE.as_f32(), ICON_SIZE.as_f32())
        .translate(ICON_OFFSET, ICON_OFFSET);
    let paint = PaintBuilder::new()
        .set_color(Color::grayscale_f01(0.3))
        .set_style(namui::PaintStyle::Stroke);

    path(poly_selection_path, paint)
}

fn render_hand_icon() -> RenderingTree {
    let hand_path = get_hand_path()
        .scale(ICON_SIZE.as_f32(), ICON_SIZE.as_f32())
        .translate(ICON_OFFSET, ICON_OFFSET);
    let paint = PaintBuilder::new()
        .set_color(Color::grayscale_f01(0.3))
        .set_style(namui::PaintStyle::Stroke);

    path(hand_path, paint)
}

fn render_zoom_icon() -> RenderingTree {
    let zoom_path = get_zoom_path()
        .scale(ICON_SIZE.as_f32(), ICON_SIZE.as_f32())
        .translate(ICON_OFFSET, ICON_OFFSET);
    let paint = PaintBuilder::new()
        .set_color(Color::grayscale_f01(0.3))
        .set_style(namui::PaintStyle::Stroke);

    path(zoom_path, paint)
}

fn get_poly_selection_path() -> PathBuilder {
    PathBuilder::new()
        .move_to(px(1.0), px(1.0))
        .line_to(px(0.8), px(0.8))
        .line_to(px(0.2), px(0.6))
        .line_to(px(0.0), px(0.2))
        .line_to(px(0.5), px(0.0))
        .line_to(px(0.7), px(0.2))
        .line_to(px(0.8), px(0.8))
        .close()
}

fn get_hand_path() -> PathBuilder {
    PathBuilder::new()
        .arc_to(
            Rect::Ltrb {
                left: px(0.0625),
                top: px(0.5625),
                right: px(0.1875),
                bottom: px(0.6875),
            },
            Angle::Radian(2.251705961447832),
            Angle::Radian(2.0 * PI),
        )
        .line_to(px(0.25), px(0.6458333333333333))
        .line_to(px(0.25), px(0.1875))
        .arc_to(
            Rect::Ltrb {
                left: px(0.25),
                top: px(0.125),
                right: px(0.375),
                bottom: px(0.25),
            },
            Angle::Radian(2.0 * PI),
            Angle::Radian(2.0 * PI),
        )
        .line_to(px(0.375), px(0.4375))
        .line_to(px(0.375), px(0.0625))
        .arc_to(
            Rect::Ltrb {
                left: px(0.375),
                top: px(0.0),
                right: px(0.5),
                bottom: px(0.125),
            },
            Angle::Radian(2.0 * PI),
            Angle::Radian(2.0 * PI),
        )
        .line_to(px(0.5), px(0.4375))
        .line_to(px(0.5), px(0.125))
        .arc_to(
            Rect::Ltrb {
                left: px(0.5),
                top: px(0.0625),
                right: px(0.625),
                bottom: px(0.1875),
            },
            Angle::Radian(2.0 * PI),
            Angle::Radian(2.0 * PI),
        )
        .line_to(px(0.625), px(0.5))
        .line_to(px(0.625), px(0.3125))
        .arc_to(
            Rect::Ltrb {
                left: px(0.625),
                top: px(0.25),
                right: px(0.75),
                bottom: px(0.375),
            },
            Angle::Radian(2.0 * PI),
            Angle::Radian(2.0 * PI),
        )
        .line_to(px(0.75), px(0.6875))
        .arc_to(
            Rect::Ltrb {
                left: px(0.25),
                top: px(0.4375),
                right: px(0.75),
                bottom: px(0.9375),
            },
            Angle::Radian(0.0),
            Angle::Radian(2.251705961447832),
        )
        .close()
}

fn get_zoom_path() -> PathBuilder {
    PathBuilder::new()
        .move_to(px(1.0), px(1.0))
        .line_to(px(0.6), px(0.6))
        .arc_to(
            Rect::Ltrb {
                left: px(0.0),
                top: px(0.0),
                right: px(0.6),
                bottom: px(0.6),
            },
            Angle::Radian(PI / 4.0),
            Angle::Radian(-PI * 2.0),
        )
        .close()
}
