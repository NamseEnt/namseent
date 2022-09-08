use super::*;

pub struct Props {}

impl CharacterEditModal {
    pub fn render_label_filter(&self, props: Props) -> RenderingTree {
        let title = translate(
            12.px(),
            12.px(),
            typography::title::left_top("Select labels", Color::WHITE),
        );
        title
    }
}
