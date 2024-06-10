use crate::components::*;
use namui::*;
use namui_prebuilt::simple_rect;
use rpc::data::*;

pub struct Viewer {
    sequence_player: Option<sequence_player::SequencePlayer>,
    start_index: usize,
}

enum Event {
    DataForSequencePlayerLoaded {
        sequence: Sequence,
        project_shared_data: ProjectSharedData,
    },
}

impl Viewer {
    pub fn new(sequence_id: Uuid, start_index: usize) -> Self {
        spawn_local(async move {
            let rpc::get_sequence_and_project_shared_data::Response {
                sequence,
                project_shared_data_json,
            }: rpc::get_sequence_and_project_shared_data::Response = crate::RPC
                .get_sequence_and_project_shared_data(
                    rpc::get_sequence_and_project_shared_data::Request { sequence_id },
                )
                .await
                .unwrap();

            namui::event::send(Event::DataForSequencePlayerLoaded {
                sequence,
                project_shared_data: serde_json::from_str(&project_shared_data_json).unwrap(),
            });
        });
        Self {
            sequence_player: None,
            start_index,
        }
    }
}

impl namui::Entity for Viewer {
    type Props = ();

    fn update(&mut self, event: &namui::Event) {
        event.is::<Event>(|event| match event {
            Event::DataForSequencePlayerLoaded {
                sequence,
                project_shared_data,
            } => {
                self.sequence_player = Some(sequence_player::SequencePlayer::new(
                    sequence.clone(),
                    project_shared_data.clone(),
                    self.start_index,
                    None,
                ));
            }
        });

        self.sequence_player.as_mut().map(|sequence_player| {
            sequence_player.update(event);
        });
    }
    fn render(&self, _: &Self::Props) -> namui::RenderingTree {
        let wh = namui::screen::size();
        let background = simple_rect(wh, Color::TRANSPARENT, 0.px(), Color::BLACK);

        render([
            background,
            match &self.sequence_player {
                Some(sequence_player) => sequence_player.render(sequence_player::Props { wh }),
                None => namui::RenderingTree::Empty,
            },
        ])
    }
}
