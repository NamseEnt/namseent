use super::*;
use namui::types::PixelSize;
use std::collections::BTreeSet;

impl WysiwygWindow {
    pub(super) fn render_moving_path(&self, layer: &Layer) -> namui::RenderingTree {
        let time_and_xys: Vec<(Time, Xy<PixelSize>)> = {
            let mut result = vec![];

            let times = layer
                .image
                .get_keyframe_infos()
                .iter()
                .map(|info| info.time)
                .collect::<BTreeSet<_>>();

            let get_xy = |time: Time| -> Option<Xy<PixelSize>> {
                let x = layer.image.x.get_value(time)?;
                let y = layer.image.y.get_value(time)?;
                Some(Xy { x, y })
            };
            for time in times {
                if let Some(xy) = get_xy(time) {
                    result.push((time, xy))
                }
            }

            result
        };

        let mut path_builder = PathBuilder::new();
        let paint_builder = PaintBuilder::new()
            .set_anti_alias(true)
            .set_style(PaintStyle::Stroke)
            .set_color(Color::grayscale_f01(0.0))
            .set_stroke_width(1.0);

        if let Some((_, xy)) = time_and_xys.first() {
            path_builder = path_builder.move_to(xy.x.into(), xy.y.into());
        }

        for (_, xy) in time_and_xys.iter().skip(1) {
            path_builder = path_builder.line_to(xy.x.into(), xy.y.into());
        }

        path(path_builder, paint_builder)
    }
}
