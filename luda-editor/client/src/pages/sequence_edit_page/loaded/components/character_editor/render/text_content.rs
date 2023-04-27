use super::*;
use namui_prebuilt::typography::body::center_top;

impl CharacterEditor {
    pub fn render_text_content(
        &self,
        wh: Wh<Px>,
        content: impl AsRef<str>,
    ) -> namui::RenderingTree {
        table::padding(8.px(), |wh| {
            center_top(wh.width, content, color::STROKE_NORMAL)
        })(wh)
    }
}
