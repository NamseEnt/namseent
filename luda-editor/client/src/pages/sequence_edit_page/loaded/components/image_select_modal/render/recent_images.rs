use super::*;

pub struct Props {
    pub wh: Wh<Px>,
}

impl ImageSelectModal {
    pub fn render_recent_images(&self, props: Props) -> namui::RenderingTree {
        let title = translate(
            12.px(),
            12.px(),
            typography::title::left_top("Recent", Color::WHITE),
        );

        // TODO

        render([
            simple_rect(props.wh, Color::WHITE, 1.px(), Color::BLACK),
            title,
        ])
    }
}
