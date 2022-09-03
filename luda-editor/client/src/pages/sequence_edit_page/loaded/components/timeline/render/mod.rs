mod context_menu;
mod image_track;
mod time_ruler;

use super::*;
use namui_prebuilt::{table::TableCell, *};

impl Timeline {
    pub fn render(&self, props: Props) -> namui::RenderingTree {
        render([
            simple_rect(props.wh, Color::WHITE, 1.px(), Color::TRANSPARENT),
            table::vertical([
                self.render_time_ruler(), // TODO: Render audio track
                self.render_image_track(props.cut),
            ])(props.wh),
            self.render_context_menu(),
        ])
    }
    fn render_time_ruler(&self) -> TableCell {
        table::fixed(30.px(), |wh| {
            time_ruler::render_time_ruler(time_ruler::TimeRulerProps {
                rect: Rect::from_xy_wh(Xy::zero(), wh),
                start_at: self.start_at,
                time_per_px: self.time_per_px,
            })
        })
    }
}
