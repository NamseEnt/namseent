use super::*;

pub fn render_grid_guide(wh: Wh<Px>) -> RenderingTree {
    let paint = PaintBuilder::new()
        .set_style(PaintStyle::Stroke)
        .set_color(Color::from_f01(0.5, 0.5, 0.5, 0.5))
        .set_stroke_width(5.px());

    let horizontal_third = (0..2).map(|index| {
        let x = wh.width * (index + 1) as f32 / 3.0;
        PathBuilder::new().move_to(x, 0.px()).line_to(x, wh.height)
    });
    let vertical_third = (0..2).map(|index| {
        let y = wh.height * (index + 1) as f32 / 3.0;
        PathBuilder::new().move_to(0.px(), y).line_to(wh.width, y)
    });

    let top = PathBuilder::new()
        .move_to(wh.width / 2.0, 0.px())
        .line_to(wh.width / 2.0, wh.height / 20.0);
    let bottom = PathBuilder::new()
        .move_to(wh.width / 2.0, wh.height)
        .line_to(wh.width / 2.0, wh.height - wh.height * 1.0 / 20.0);
    let left = PathBuilder::new()
        .move_to(0.px(), wh.height / 2.0)
        .line_to(wh.width / 20.0, wh.height / 2.0);
    let right = PathBuilder::new()
        .move_to(wh.width, wh.height / 2.0)
        .line_to(wh.width - wh.width * 1.0 / 20.0, wh.height / 2.0);

    let center_vertical = PathBuilder::new()
        .move_to(wh.width / 2.0 - wh.width / 20.0, wh.height / 2.0)
        .line_to(wh.width / 2.0 + wh.width / 20.0, wh.height / 2.0);
    let center_horizontal = PathBuilder::new()
        .move_to(wh.width / 2.0, wh.height / 2.0 - wh.height / 20.0)
        .line_to(wh.width / 2.0, wh.height / 2.0 + wh.height / 20.0);

    let paths = [top, bottom, left, right, center_vertical, center_horizontal]
        .into_iter()
        .chain(horizontal_third)
        .chain(vertical_third);

    render(paths.map(|path| namui::path(path, paint.clone())))
}
