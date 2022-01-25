mod events;
mod go_back_button;
use namui::{render, Color, RenderingTree, Wh, XywhRect};

use crate::app::{events::RouterEvent, sequence_list::SequenceList};

use self::{events::TopBarEvent, go_back_button::render_go_back_button};

const MARGIN: f32 = 4.0;

pub struct TopBarProps {
    pub xywh: XywhRect<f32>,
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

        namui::translate(
            props.xywh.x,
            props.xywh.y,
            render![
                border,
                namui::translate(MARGIN, MARGIN, render_go_back_button(go_back_button_wh)),
            ],
        )
    }
}

fn change_page_to_sequence_list() {
    namui::event::send(RouterEvent::PageChangeToSequenceListEvent(Box::new(
        move |app_context| SequenceList::new(app_context.socket.clone()),
    )))
}
