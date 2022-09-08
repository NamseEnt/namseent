use super::*;

pub struct Props {}

impl CharacterEditModal {
    pub fn render_recently_used(&self, props: Props) -> RenderingTree {
        let title = translate(
            12.px(),
            12.px(),
            typography::title::left_top("Recently used", Color::WHITE),
        );
        // TODO: Show recently used images

        title
    }
}
