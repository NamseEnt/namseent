use self::{
    events::SequenceListEvent,
    types::{SequenceLoadStateMap, SequenceTitlesLoadState},
};
use luda_editor_rpc::Socket;
use namui::{render, Entity, Wh};
use std::collections::HashMap;
mod button_text;
mod events;
mod list;
mod list_item;
mod open_button;
mod ops;
mod reload_titles_button;
mod rounded_rectangle;
mod title_button;
mod types;

const LIST_WIDTH: f32 = 800.0;
const BUTTON_HEIGHT: f32 = 36.0;
const RECT_RADIUS: f32 = 4.0;
const SPACING: f32 = 4.0;
const MARGIN: f32 = 4.0;

pub struct SequenceListProps {
    pub wh: Wh<f32>,
}

pub struct SequenceList {
    sequence_load_state_map: SequenceLoadStateMap,
    sequence_titles_load_state: Option<SequenceTitlesLoadState>,
    socket: Socket,
    scroll_y: f32,
}

impl SequenceList {
    pub fn new(socket: Socket) -> Self {
        let mut sequence_list = Self {
            sequence_load_state_map: HashMap::new(),
            sequence_titles_load_state: None,
            socket,
            scroll_y: 0.0,
        };
        sequence_list.load_sequence_titles();
        sequence_list
    }
}

impl Entity for SequenceList {
    type Props = SequenceListProps;

    fn update(&mut self, event: &dyn std::any::Any) {
        if let Some(event) = event.downcast_ref::<SequenceListEvent>() {
            match event {
                SequenceListEvent::SequenceLoadStateUpdateEvent { path, state } => match state {
                    Some(state) => {
                        if let Some(old_state) = self.sequence_load_state_map.get(path) {
                            if old_state.started_at > state.started_at {
                                return;
                            }
                        }

                        self.sequence_load_state_map
                            .insert(path.clone(), (*state).clone());
                    }
                    None => {
                        self.sequence_load_state_map.remove(path);
                    }
                },
                SequenceListEvent::SequenceTitleButtonClickedEvent { path } => {
                    let should_clear_load_state = self.sequence_load_state_map.get(path).is_some();
                    match should_clear_load_state {
                        true => clear_sequence(path),
                        false => self.load_sequence(path),
                    }
                }
                SequenceListEvent::SequenceTitlesLoadStateUpdateEvent { state } => {
                    if let Some(old_state) = &self.sequence_titles_load_state {
                        if old_state.started_at > state.started_at {
                            return;
                        }
                    }

                    self.sequence_titles_load_state = Some(state.clone());
                }
                SequenceListEvent::SequenceReloadTitlesButtonClickedEvent => {
                    self.load_sequence_titles()
                }
                SequenceListEvent::ScrolledEvent { scroll_y } => {
                    self.scroll_y = *scroll_y;
                }
            }
        }
    }

    fn render(&self, props: &Self::Props) -> namui::RenderingTree {
        let list_wh = Wh {
            width: LIST_WIDTH,
            height: props.wh.height - 2.0 * MARGIN - SPACING - BUTTON_HEIGHT,
        };
        render![
            namui::translate(
                MARGIN,
                MARGIN,
                self.render_reload_titles_button(Wh {
                    width: LIST_WIDTH,
                    height: BUTTON_HEIGHT
                })
            ),
            namui::translate(
                MARGIN,
                MARGIN + SPACING + BUTTON_HEIGHT,
                self.render_list(list_wh)
            ),
        ]
    }
}

fn clear_sequence(path: &String) {
    namui::event::send(SequenceListEvent::SequenceLoadStateUpdateEvent {
        path: path.clone(),
        state: None,
    })
}
