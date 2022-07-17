mod events;
mod go_back_button;
mod meta_update_button;
mod saving_status_text;
mod sheet_sequence_syncer_bar;

use self::{events::TopBarEvent, go_back_button::render_go_back_button};
use super::{
    sequence_saver::SequenceSaverStatus, sheet_sequence_syncer::SheetSequenceSyncerStatus,
};
use crate::app::{events::RouterEvent, sequence_list::SequenceList};
use meta_update_button::*;
use namui::prelude::*;
use saving_status_text::*;
use sheet_sequence_syncer_bar::*;

const MARGIN: Px = px(4.0);

pub struct TopBarProps<'a> {
    pub rect: Rect<Px>,
    pub sequence_saver_status: &'a SequenceSaverStatus,
    pub sheet_sequence_syncer_status: &'a SheetSequenceSyncerStatus,
}

pub struct TopBar {}

impl TopBar {
    pub fn new() -> Self {
        Self {}
    }

    pub fn update(&mut self, event: &dyn std::any::Any) {
        if let Some(event) = event.downcast_ref::<TopBarEvent>() {
            match event {
                TopBarEvent::GoBackButtonClicked => change_page_to_sequence_list(),
            }
        }
    }

    pub fn render(&self, props: &TopBarProps) -> RenderingTree {
        let go_back_button_wh = Wh {
            width: px(64.0),
            height: props.rect.height() - 2.0 * MARGIN,
        };

        let border = namui::rect(namui::RectParam {
            rect: Rect::Xywh {
                x: px(0.0),
                y: px(0.0),
                width: props.rect.width(),
                height: props.rect.height(),
            },
            style: namui::RectStyle {
                stroke: Some(namui::RectStroke {
                    color: Color::BLACK,
                    border_position: namui::BorderPosition::Inside,
                    width: px(1.0),
                }),
                ..Default::default()
            },
        });

        let meta_update_button_wh = Wh {
            width: px(108.0),
            height: props.rect.height() - 2.0 * MARGIN,
        };

        namui::translate(
            props.rect.x(),
            props.rect.y(),
            render([
                border,
                namui::translate(MARGIN, MARGIN, render_go_back_button(go_back_button_wh)),
                namui::translate(
                    go_back_button_wh.width + MARGIN * 2.0,
                    MARGIN,
                    render_saving_status_text(&SavingStatusTextProps {
                        height: props.rect.height() - 2.0 * MARGIN,
                        sequence_saver_status: props.sequence_saver_status,
                    }),
                ),
                namui::translate(
                    props.rect.width() - MARGIN * 2.0 - meta_update_button_wh.width,
                    MARGIN,
                    render_sheet_sequence_syncer_bar(&SheetSequenceSyncerBarProps {
                        height: props.rect.height() - 2.0 * MARGIN,
                        syncer_status: props.sheet_sequence_syncer_status,
                    }),
                ),
                namui::translate(
                    props.rect.width() - MARGIN - meta_update_button_wh.width,
                    MARGIN,
                    render_meta_update_button(&MetaUpdateButtonProps {
                        wh: meta_update_button_wh,
                    }),
                ),
            ]),
        )
    }
}

fn change_page_to_sequence_list() {
    namui::event::send(RouterEvent::PageChangeToSequenceListEvent(Box::new(
        move |context| SequenceList::new(context.storage.clone()),
    )))
}
