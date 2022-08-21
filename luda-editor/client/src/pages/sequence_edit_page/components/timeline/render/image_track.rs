use super::*;
use crate::storage::Cut;
use namui_prebuilt::{table::TableCell, *};

impl Timeline {
    pub fn render_image_track<'a>(&'a self, cut: &'a Cut) -> TableCell<'a> {
        table::ratio(1.0, |wh| {
            let mut clips = vec![];

            let mut last_right = 0.px();
            for image_clip in cut.image_clips.iter() {
                let is_selected = self
                    .selected_clip_ids
                    .iter()
                    .any(|id| id == image_clip.id());
                let duration = match &self.clip_sash_dragging {
                    Some(dragging) if dragging.clip_id == image_clip.id() => {
                        image_clip.duration
                            + (dragging.global_mouse_x - dragging.start_global_mouse_x)
                                * self.time_per_px
                    }
                    _ => image_clip.duration,
                };
                clips.push(translate(
                    last_right,
                    0.px(),
                    resizable_clip::render_resizable_clip(&resizable_clip::Props {
                        id: image_clip.id(),
                        duration,
                        is_selected,
                        is_single_selected: is_selected && self.selected_clip_ids.len() == 1,
                        time_per_px: self.time_per_px,
                        track_body_wh: wh,
                    }),
                ));
                let clip_width = duration / self.time_per_px;
                last_right += clip_width;
            }

            render([
                simple_rect(wh, Color::WHITE, 1.px(), Color::TRANSPARENT).attach_event(|builder| {
                    builder.on_mouse_down_in(|event| {
                        if event.button == Some(MouseButton::Right) {
                            namui::event::send(Event::OpenContextMenu(ContextMenu::ImageClip {
                                global_xy: event.global_xy,
                            }))
                        }
                    });
                }),
                render(clips),
            ])
        })
    }
}
