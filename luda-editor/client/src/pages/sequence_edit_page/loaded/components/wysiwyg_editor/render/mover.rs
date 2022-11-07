use super::*;
use rpc::data::Circumscribed;

#[derive(Debug, Clone, Copy)]
pub struct MoverDraggingContext {
    pub start_global_xy: Xy<Px>,
    pub end_global_xy: Xy<Px>,
    pub container_wh: Wh<Px>,
}

impl MoverDraggingContext {
    pub fn move_circumscribed(
        &self,
        circumscribed: Circumscribed<Percent>,
    ) -> Circumscribed<Percent> {
        let delta_xy = self.end_global_xy - self.start_global_xy;
        let delta_xy_percent: Xy<Percent> = Xy::new(
            (delta_xy.x / self.container_wh.width).into(),
            (delta_xy.y / self.container_wh.height).into(),
        );
        Circumscribed {
            center_xy: circumscribed.center_xy + delta_xy_percent,
            ..circumscribed
        }
    }
}

impl WysiwygEditor {
    pub fn render_border_with_move_handling(
        &self,
        image_dest_rect: Rect<Px>,
        container_wh: Wh<Px>,
    ) -> RenderingTree {
        translate(
            image_dest_rect.x(),
            image_dest_rect.y(),
            simple_rect(
                Wh {
                    width: image_dest_rect.width(),
                    height: image_dest_rect.height(),
                },
                Color::grayscale_f01(0.2),
                px(2.0),
                Color::TRANSPARENT,
            )
            .with_mouse_cursor({
                let is_dragging = matches!(self.dragging, Some(Dragging::Mover { .. }));
                if is_dragging {
                    namui::MouseCursor::Move
                } else {
                    namui::MouseCursor::Pointer
                }
            })
            .attach_event(move |builder| {
                builder.on_mouse_down_in(move |event| {
                    event.stop_propagation();
                    namui::event::send(InternalEvent::ImageMoveStart {
                        start_global_xy: event.global_xy,
                        end_global_xy: event.global_xy,
                        container_wh,
                    });
                });
            }),
        )
    }
}
