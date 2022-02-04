use crate::app::types::*;
use namui::prelude::*;

// NOTE : I think context menu should divided into two parts, one is the view and one is the contents with context.
pub(super) struct ContextMenu {
    id: String,
    xy: Xy<f32>,
    mouse_position_in_time: Time,
}

pub(super) enum ContextMenuEvent {
    CloseContextMenu(String),
    CreateCameraClip(Time),
}

pub(super) struct ContextMenuProps {}

impl ContextMenu {
    pub fn new(xy: &Xy<f32>, mouse_position_in_time: &Time) -> Self {
        let id = namui::nanoid();
        Self {
            id,
            xy: *xy,
            mouse_position_in_time: *mouse_position_in_time,
        }
    }
    pub fn get_id(&self) -> &str {
        &self.id
    }
    pub fn update(&mut self, event: &dyn std::any::Any) {
        if let Some(event) = event.downcast_ref::<NamuiEvent>() {
            match event {
                NamuiEvent::MouseDown(mouse_event) => {
                    let context_menu_wh = self.wh();
                    let mouse_relative_position = mouse_event.xy - self.xy;
                    let is_out_of_context_menu = mouse_relative_position.x < 0.0
                        || mouse_relative_position.y < 0.0
                        || mouse_relative_position.x > context_menu_wh.width
                        || mouse_relative_position.y > context_menu_wh.height;

                    if is_out_of_context_menu {
                        namui::event::send(ContextMenuEvent::CloseContextMenu(self.id.clone()));
                    }
                }
                _ => {}
            }
        }
    }

    pub fn render(&self, props: &ContextMenuProps) -> RenderingTree {
        // TODO : Fix this code using render_open_button things
        namui::absolute(
            self.xy.x,
            self.xy.y,
            render![
                rect(RectParam {
                    x: 0.0,
                    y: 0.0,
                    width: self.wh().width,
                    height: self.wh().height,
                    style: RectStyle {
                        stroke: Some(RectStroke {
                            border_position: BorderPosition::Middle,
                            color: Color::BLACK,
                            width: 1.0,
                        }),
                        fill: Some(RectFill {
                            color: Color::WHITE
                        }),
                        ..Default::default()
                    },
                })
                .attach_event(move |builder| {
                    let time_to_create_clip = self.mouse_position_in_time;
                    let id = self.id.clone();
                    builder.on_mouse_up(move |event| {
                        if event.button == Some(MouseButton::Left) {
                            namui::event::send(ContextMenuEvent::CloseContextMenu(id.clone()));
                            namui::event::send(ContextMenuEvent::CreateCameraClip(
                                time_to_create_clip,
                            ));
                        }
                    })
                }),
                text(TextParam {
                    x: 10.0,
                    y: 12.5,
                    text: "New clip".to_string(),
                    align: TextAlign::Left,
                    baseline: TextBaseline::Middle,
                    font_type: FontType {
                        language: Language::Ko,
                        serif: false,
                        font_weight: FontWeight::REGULAR,
                        size: 12,
                    },
                    style: TextStyle {
                        color: Color::BLACK,
                        ..Default::default()
                    },
                }),
            ],
        )
    }

    fn wh(&self) -> Wh<f32> {
        Wh {
            width: 100.0,
            height: 25.0,
        }
    }
}
