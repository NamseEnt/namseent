use namui::{absolute, Color, NamuiEvent, RectParam, RectStroke, RectStyle, RenderingTree, Xy};

const ICON_SIZE: f32 = 16.0;
const ICON_OFFSET: f32 = 8.0;

pub struct Tool {
    last_cursor_position: Xy<f32>,
    current_tool_type: ToolType,
}
impl Tool {
    pub fn new() -> Self {
        Self {
            last_cursor_position: Xy { x: 0.0, y: 0.0 },
            current_tool_type: ToolType::RectSelection,
        }
    }

    pub fn render_cursor_icon(&self) -> RenderingTree {
        absolute(
            self.last_cursor_position.x,
            self.last_cursor_position.y,
            match self.current_tool_type {
                ToolType::RectSelection => render_rect_selection_icon(),
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

    pub fn get_current_tool_type(&self) -> &ToolType {
        &self.current_tool_type
    }
}

#[derive(Clone, Copy)]
pub enum ToolType {
    RectSelection,
}

fn render_rect_selection_icon() -> RenderingTree {
    namui::rect(RectParam {
        x: ICON_OFFSET,
        y: ICON_OFFSET,
        width: ICON_SIZE,
        height: ICON_SIZE,
        style: RectStyle {
            stroke: Some(RectStroke {
                color: Color::grayscale_f01(0.3),
                width: 2.0,
                border_position: namui::BorderPosition::Inside,
            }),
            fill: None,
            round: None,
        },
    })
}
