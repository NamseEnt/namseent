use super::*;

pub struct Props {}

impl CharacterEditModal {
    pub fn render_filtered_image_list(&self, props: Props) -> RenderingTree {
        let title = translate(
            12.px(),
            12.px(),
            typography::title::left_top("Select image", Color::WHITE),
        );
        title
    }
}
