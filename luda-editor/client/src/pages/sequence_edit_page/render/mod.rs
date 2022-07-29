mod line_list;
mod top_bar;

use super::*;
use editor_core::storage::SyncStatus;
use namui_prebuilt::*;

pub struct Props {
    pub wh: Wh<Px>,
    pub sync_send_status: SyncStatus,
}

impl SequenceEditPage {
    pub fn render(&self, props: Props) -> namui::RenderingTree {
        let system_tree = self.editor_history_system.get_state();
        let sequence = system_tree
            .sequence_list
            .iter()
            .find(|sequence| sequence.id() == self.selected_sequence_id)
            .unwrap();

        table::vertical([
            table::fixed(20.px(), |wh| {
                self.render_top_bar(wh, sequence, props.sync_send_status)
            }),
            table::ratio(
                1.0,
                table::horizontal([table::ratio(1.0, |wh| self.render_line_list(wh, sequence))]),
            ),
        ])(props.wh)
    }
}
