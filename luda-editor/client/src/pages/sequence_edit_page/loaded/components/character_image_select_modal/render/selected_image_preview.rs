use super::*;

pub struct Props {}

impl CharacterEditModal {
    pub fn render_selected_image_preview(&self, props: Props) -> RenderingTree {
        let title = translate(
            12.px(),
            12.px(),
            typography::title::left_top("Preview", Color::WHITE),
        );
        title
    }
}
