use crate::app::types::*;
use namui::prelude::*;

pub struct PlayHeadProps<'a> {
    pub playback_time: &'a Time,
    pub start_at: &'a Time,
    pub time_per_pixel: &'a TimePerPixel,
    pub time_ruler_height: f32,
    pub track_body_height: f32,
}

pub fn render_play_head(props: &PlayHeadProps) -> namui::RenderingTree {
    let center_x = ((props.playback_time - props.start_at) / props.time_per_pixel).into_f32();

    let path = PathBuilder::new()
        .move_to(-props.time_ruler_height / 2.0, 0.0)
        .line_to(props.time_ruler_height / 2.0, 0.0)
        .line_to(0.5, props.time_ruler_height)
        .line_to(0.5, props.time_ruler_height + props.track_body_height)
        .line_to(-0.5, props.time_ruler_height + props.track_body_height)
        .line_to(-0.5, props.time_ruler_height)
        .close();
    let paint = PaintBuilder::new()
        .set_color(Color::RED)
        .set_style(PaintStyle::Fill)
        .set_anti_alias(true);

    translate(center_x, 0.0, namui::path(path, paint))
}
