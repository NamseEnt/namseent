mod line_list;
mod top_bar;

use super::*;
use namui_prebuilt::*;

pub struct Props {
    pub wh: Wh<Px>,
}

impl LoadedSequenceEditorPage {
    pub fn render(&self, props: Props) -> namui::RenderingTree {
        let system_tree = self.editor_history_system.get_state();
        let sequence = system_tree.sequence;

        table::vertical([
            table::fixed(20.px(), |wh| {
                self.render_top_bar(wh, &sequence, self.syncer.get_sync_status())
            }),
            table::ratio(
                1.0,
                table::horizontal([table::ratio(1.0, |wh| self.render_line_list(wh, &sequence))]),
            ),
        ])(props.wh)
    }
}
