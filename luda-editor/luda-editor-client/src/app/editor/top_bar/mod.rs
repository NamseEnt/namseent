mod events;
mod go_back_button;
use self::{events::TopBarEvent, go_back_button::render_go_back_button};
use crate::app::{events::RouterEvent, sequence_list::SequenceList};
use namui::{render, Color, RenderingTree, Wh, XywhRect};
mod saving_status_text;
use super::{
    sequence_saver::SequenceSaverStatus, sheet_sequence_syncer::SheetSequenceSyncerStatus,
};
use saving_status_text::*;
mod sheet_sequence_syncer_bar;
use sheet_sequence_syncer_bar::*;
mod meta_update_button;
use meta_update_button::*;

const MARGIN: f32 = 4.0;

pub struct TopBarProps<'a> {
    pub xywh: XywhRect<f32>,
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
                _ => {}
            }
        }
    }

    pub fn render(&self, props: &TopBarProps) -> RenderingTree {
        let go_back_button_wh = Wh {
            width: 64.0,
            height: props.xywh.height - 2.0 * MARGIN,
        };

        let border = namui::rect(namui::RectParam {
            x: 0.0,
            y: 0.0,
            width: props.xywh.width,
            height: props.xywh.height,
            style: namui::RectStyle {
                stroke: Some(namui::RectStroke {
                    color: Color::BLACK,
                    border_position: namui::BorderPosition::Inside,
                    width: 1.0,
                }),
                ..Default::default()
            },
        });

        let meta_update_button_wh = Wh {
            width: 108.0,
            height: props.xywh.height - 2.0 * MARGIN,
        };

        namui::translate(
            props.xywh.x,
            props.xywh.y,
            render![
                border,
                namui::translate(MARGIN, MARGIN, render_go_back_button(go_back_button_wh)),
                namui::translate(
                    go_back_button_wh.width + MARGIN * 2.0,
                    MARGIN,
                    render_saving_status_text(&SavingStatusTextProps {
                        height: props.xywh.height - 2.0 * MARGIN,
                        sequence_saver_status: props.sequence_saver_status,
                    }),
                ),
                namui::translate(
                    props.xywh.width - MARGIN * 2.0 - meta_update_button_wh.width,
                    MARGIN,
                    render_sheet_sequence_syncer_bar(&SheetSequenceSyncerBarProps {
                        height: props.xywh.height - 2.0 * MARGIN,
                        syncer_status: props.sheet_sequence_syncer_status,
                    }),
                ),
                namui::translate(
                    props.xywh.width - MARGIN - meta_update_button_wh.width,
                    MARGIN,
                    render_meta_update_button(&MetaUpdateButtonProps {
                        wh: meta_update_button_wh
                    }),
                ),
            ],
        )
    }
}

fn change_page_to_sequence_list() {
    namui::event::send(RouterEvent::PageChangeToSequenceListEvent(Box::new(
        move |app_context| SequenceList::new(app_context.socket.clone()),
    )))
}
