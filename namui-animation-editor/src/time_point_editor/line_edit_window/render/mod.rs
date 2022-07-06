use super::*;

impl LineEditWindow {
    pub fn render(&self, props: Props) -> namui::RenderingTree {
        simple_rect(props.wh, Color::BLACK, 1.px(), Color::WHITE)
    }
}
