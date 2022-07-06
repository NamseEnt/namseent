use namui::prelude::*;

pub struct PlayHeadProps {
    pub playback_time: Time,
    pub start_at: Time,
    pub time_per_px: TimePerPx,
    pub time_ruler_height: Px,
    pub track_body_height: Px,
}

pub fn render_play_head(props: &PlayHeadProps) -> namui::RenderingTree {
    let center_x = (props.playback_time - props.start_at) / props.time_per_px;

    let path = PathBuilder::new()
        .move_to(-props.time_ruler_height / 2.0, px(0.0))
        .line_to(props.time_ruler_height / 2.0, px(0.0))
        .line_to(px(0.5), props.time_ruler_height)
        .line_to(px(0.5), props.time_ruler_height + props.track_body_height)
        .line_to(px(-0.5), props.time_ruler_height + props.track_body_height)
        .line_to(px(-0.5), props.time_ruler_height)
        .close();
    let paint = PaintBuilder::new()
        .set_color(Color::RED)
        .set_style(PaintStyle::Fill)
        .set_anti_alias(true);

    translate(center_x, px(0.0), namui::path(path, paint))
}
