use super::*;
use crate::{
    pages::{router, sequence_list_page::SequenceListPage},
    sync::SyncStatus,
};
use namui_prebuilt::{button::text_button, *};
use std::sync::Arc;

impl LoadedSequenceEditorPage {
    pub fn render_top_bar(
        &self,
        wh: Wh<Px>,
        sequence: &Sequence,
        sync_send_status: SyncStatus,
    ) -> namui::RenderingTree {
        let go_back_button = table::fixed(52.px(), |wh| {
            text_button(
                Rect::from_xy_wh(Xy::zero(), wh),
                "Go back",
                Color::WHITE,
                Color::WHITE,
                1.px(),
                Color::BLACK,
                {
                    let project_id = self.project_id.clone();
                    move || {
                        let project_id = project_id.clone();
                        // TODO: Check saving finished
                        namui::event::send(router::Event::Route(Arc::new(move || {
                            router::Route::SequenceListPage(SequenceListPage::new(
                                project_id.clone(),
                            ))
                        })));
                    }
                },
            )
        });
        let sequence_name_label = table::fixed(200.px(), |wh| {
            typography::body::left(wh.height, format!("Title: {}", sequence.name), Color::WHITE)
        });
        let sync_status = table::ratio(1.0, |wh| {
            let text = match sync_send_status {
                SyncStatus::Idle => return RenderingTree::Empty,
                SyncStatus::Syncing(time) => {
                    format!(
                        "Syncing... ({})",
                        (namui::now() - time).relative_time_format()
                    )
                }
                SyncStatus::Synced(time) => {
                    format!("Synced ({})", (namui::now() - time).relative_time_format())
                }
                SyncStatus::Error(message) => {
                    format!("Error: {}", message)
                }
            };
            typography::body::left(wh.height, text, Color::WHITE)
        });
        fn margin() -> table::TableCell<'static> {
            table::fixed(10.px(), |_wh| RenderingTree::Empty)
        }
        render([
            simple_rect(wh, Color::WHITE, 1.px(), Color::TRANSPARENT),
            table::horizontal([
                go_back_button,
                margin(),
                sequence_name_label,
                margin(),
                sync_status,
            ])(wh),
        ])
    }
}
