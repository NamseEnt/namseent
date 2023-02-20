use super::*;
use crate::{components::sync::SyncStatus, pages::router::Router};
use namui_prebuilt::{
    button::{text_button, text_button_fit},
    *,
};

impl LoadedSequenceEditorPage {
    pub fn render_top_bar_for_player(&self, wh: Wh<Px>) -> RenderingTree {
        render([
            simple_rect(wh, Color::WHITE, 1.px(), Color::TRANSPARENT),
            text_button_fit(
                wh.height,
                "Close Player",
                Color::WHITE,
                Color::WHITE,
                1.px(),
                Color::BLACK,
                8.px(),
                [MouseButton::Left],
                move |_| {
                    namui::event::send(Event::ClosePlayer);
                },
            ),
        ])
    }

    pub fn render_top_bar_for_editor(
        &self,
        wh: Wh<Px>,
        sequence: &Sequence,
        sync_send_status: SyncStatus,
    ) -> RenderingTree {
        let project_id = self.project_id();
        let go_back_button = table::fixed(52.px(), |wh| {
            text_button(
                Rect::from_xy_wh(Xy::zero(), wh),
                "Go back",
                Color::WHITE,
                Color::WHITE,
                1.px(),
                Color::BLACK,
                [MouseButton::Left],
                {
                    move |_| {
                        // TODO: Check saving finished
                        Router::move_to(format!("/sequence_list/{project_id}"));
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
        let image_manager_button = table::fit(
            table::FitAlign::CenterMiddle,
            text_button_fit(
                wh.height,
                "Image Manager",
                Color::WHITE,
                Color::WHITE,
                1.px(),
                Color::BLACK,
                8.px(),
                [MouseButton::Left],
                |_| namui::event::send(Event::ImageManagerButtonClicked),
            ),
        );
        let download_button = table::fit(
            table::FitAlign::CenterMiddle,
            text_button_fit(
                wh.height,
                "Download",
                Color::WHITE,
                Color::WHITE,
                1.px(),
                Color::BLACK,
                8.px(),
                [MouseButton::Left],
                |_| namui::event::send(Event::DownloadButtonClicked),
            ),
        );
        let preview_button = table::fit(
            table::FitAlign::CenterMiddle,
            text_button_fit(
                wh.height,
                "Preview",
                Color::WHITE,
                Color::WHITE,
                1.px(),
                Color::BLACK,
                8.px(),
                [MouseButton::Left],
                |_| namui::event::send(Event::PreviewButtonClicked),
            ),
        );
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
                image_manager_button,
                download_button,
                preview_button,
            ])(wh),
        ])
    }
}
