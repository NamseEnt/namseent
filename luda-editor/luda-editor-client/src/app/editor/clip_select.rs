use super::*;

impl Editor {
    pub(super) fn get_single_selected_clip(&self) -> Option<Clip> {
        self.selected_clip_ids
            .iter()
            .next()
            .and_then(|id| self.get_sequence().get_clip(id))
    }
    pub(super) fn select_only_this_clip(&mut self, clip_id: &str) {
        self.update_selected_clips(&mut [clip_id].into_iter());
    }
    pub(super) fn multi_select_clip(&mut self, clip_id: &str) {
        let mut selected_clip_ids = (*self.selected_clip_ids).clone();

        if selected_clip_ids.is_empty() {
            selected_clip_ids.insert(clip_id.to_string());
        } else if !selected_clip_ids.contains(clip_id) {
            let sequence = self.get_sequence().clone();
            let selected_clip_track = self.get_selected_clip_track().unwrap();
            let selecting_clip_track = sequence.find_track_by_clip_id(clip_id).unwrap();

            if selected_clip_track.get_id() != selecting_clip_track.get_id() {
                selected_clip_ids.clear();
            }
            selected_clip_ids.insert(clip_id.to_string());
        }

        self.update_selected_clips(&mut selected_clip_ids.into_iter());
    }
    pub(super) fn deselect_all_clips(&mut self) {
        self.update_selected_clips(&mut ([] as [String; 0]).into_iter());
    }
    pub(super) fn deselect_clips<T: AsRef<str>>(&mut self, clip_ids: &[T]) {
        let mut selected_clip_ids = (*self.selected_clip_ids).clone();

        for clip_id in clip_ids {
            selected_clip_ids.remove(clip_id.as_ref());
        }

        self.update_selected_clips(&mut selected_clip_ids.into_iter());
    }
    pub(super) fn remove_dangling_selected_clips(&mut self) {
        let sequence = self.get_sequence().clone();

        let mut clip_ids_to_remove = vec![];
        self.selected_clip_ids
            .iter()
            .filter(|clip_id| sequence.find_track_by_clip_id(clip_id).is_none())
            .for_each(|clip_id| {
                clip_ids_to_remove.push(clip_id.clone());
            });

        self.deselect_clips(&clip_ids_to_remove);
    }
    pub(super) fn is_clip_in_same_track_with_selected_clips(&self, clip_id: &str) -> bool {
        if self.selected_clip_ids.len() == 0 {
            return false;
        }

        let sequence = self.get_sequence().clone();
        let selected_clip_track = self.get_selected_clip_track().unwrap();
        let clip_track = sequence.find_track_by_clip_id(clip_id).unwrap();

        selected_clip_track.get_id() == clip_track.get_id()
    }
    pub(super) fn select_all_to_time(&mut self, time: &Time) {
        if self.selected_clip_ids.len() == 0 {
            return;
        }

        let track = self.get_selected_clip_track().unwrap();

        let clips = self
            .selected_clip_ids
            .iter()
            .map(|clip_id| track.find_clip(clip_id.as_ref()).unwrap())
            .collect::<Vec<_>>();

        let most_left_clip = clips
            .iter()
            .min_by(|a, b| a.get_start_time().partial_cmp(&b.get_start_time()).unwrap())
            .unwrap();

        let most_right_clip = clips
            .iter()
            .max_by(|a, b| {
                self.get_clip_end_time(a)
                    .partial_cmp(&self.get_clip_end_time(b))
                    .unwrap()
            })
            .unwrap();

        let mut selected_clip_ids = BTreeSet::new();
        selected_clip_ids.insert(most_left_clip.get_id().to_string());
        selected_clip_ids.insert(most_right_clip.get_id().to_string());

        let most_left_clip_start_time = most_left_clip.get_start_time();
        let most_right_clip_end_time = self.get_clip_end_time(most_right_clip);

        if most_right_clip_end_time <= time {
            for clip in track.get_clips() {
                let clip_start_time = clip.get_start_time();
                if most_left_clip_start_time <= clip_start_time && clip_start_time <= time {
                    selected_clip_ids.insert(clip.get_id().to_string());
                }
            }
        } else if time <= most_left_clip_start_time {
            for clip in track.get_clips() {
                let clip_end_time = self.get_clip_end_time(&clip);
                if time <= clip_end_time && clip_end_time <= most_right_clip_end_time {
                    selected_clip_ids.insert(clip.get_id().to_string());
                }
            }
        }

        for clip in track.get_clips() {
            if clip.get_start_time() >= most_left_clip_start_time
                && self.get_clip_end_time(&clip) <= most_right_clip_end_time
            {
                selected_clip_ids.insert(clip.get_id().to_string());
            }
        }

        self.update_selected_clips(&mut selected_clip_ids.iter());
    }
    pub(super) fn select_at_once(&mut self, direction: Direction) {
        if self.selected_clip_ids.is_empty() {
            return;
        }

        let track = &*self.get_selected_clip_track().unwrap();
        let mut selected_clip_ids = (*self.selected_clip_ids).clone();

        let start_point_to_select = match direction {
            Direction::Forward => selected_clip_ids
                .iter()
                .map(|clip_id| {
                    let clip = track.find_clip(clip_id).unwrap();
                    self.get_clip_end_time(&clip)
                })
                .max()
                .unwrap(),
            Direction::Backward => selected_clip_ids
                .iter()
                .map(|clip_id| track.find_clip(clip_id).unwrap().get_start_time())
                .min()
                .unwrap(),
        };

        let ordering = match direction {
            Direction::Forward => Ordering::Less,
            Direction::Backward => Ordering::Greater,
        };

        track.get_clips().iter().for_each(|clip| {
            let results = [
                clip.get_start_time().partial_cmp(&start_point_to_select),
                self.get_clip_end_time(&clip)
                    .partial_cmp(&start_point_to_select),
            ];

            if results.iter().all(|result| result == &Some(ordering)) {
                selected_clip_ids.insert(clip.get_id().to_string());
            }
        });

        self.update_selected_clips(&mut selected_clip_ids.into_iter());
    }
    pub(super) fn get_selected_clip_ids(&self) -> &BTreeSet<String> {
        &self.selected_clip_ids
    }
    fn update_selected_clips<T: AsRef<str>>(
        &mut self,
        clip_ids: impl std::iter::Iterator<Item = T>,
    ) {
        let next_selected_clip_ids = Arc::new(BTreeSet::<String>::from_iter(
            clip_ids.map(|id| id.as_ref().to_string()),
        ));
        let is_changed = self.selected_clip_ids.len() != next_selected_clip_ids.len() || {
            self.selected_clip_ids
                .iter()
                .any(|clip_id| !next_selected_clip_ids.contains(clip_id))
        };

        if !is_changed {
            return;
        }

        self.selected_clip_ids = next_selected_clip_ids;

        if self.selected_clip_ids.len() == 1 {
            let clip_id = self.selected_clip_ids.iter().next().unwrap();
            self.clip_editor = Some(ClipEditor::new(
                &self.get_sequence().get_clip(clip_id).unwrap(),
            ));
        } else {
            self.clip_editor = None;
        }
    }
}

#[cfg(test)]
mod tests {
    use std::array::IntoIter;
    use std::collections::HashMap;

    use crate::app::editor::sequence_player::MockSequencePlay;

    use super::super::*;
    use super::*;
    use luda_editor_rpc::response_waiter::ResponseWaiter;
    use namui::prelude::*;
    use wasm_bindgen_test::wasm_bindgen_test;

    #[test]
    #[wasm_bindgen_test]
    fn shift_click_should_not_select_crossed_back_clip() {
        // make clip 1, 2, 3
        // clip 2 and 3 is crossed.
        let clips = vec![
            mock_camera_clip("1", Time::from_ms(1000.0), Time::from_ms(2000.0)),
            mock_camera_clip("2", Time::from_ms(2000.0), Time::from_ms(3000.0)),
            mock_camera_clip("3", Time::from_ms(2500.0), Time::from_ms(3500.0)),
        ];
        let sequence = mock_sequence(clips);
        let mut editor = mock_editor(sequence);
        // select 1 clip
        editor.select_only_this_clip("1");

        // shift click 2 front
        editor.select_all_to_time(&Time::from_ms(2250.0));

        // result should be 1 and 2.
        assert_eq!(
            editor.get_selected_clip_ids().iter().collect::<Vec<_>>(),
            ["1", "2"]
        );
    }
    #[test]
    #[wasm_bindgen_test]
    fn shift_click_should_not_select_crossed_back_clip_backward() {
        // make clip 1, 2, 3
        // clip 1 and 2 is crossed.
        let clips = vec![
            mock_camera_clip("1", Time::from_ms(1000.0), Time::from_ms(2000.0)),
            mock_camera_clip("2", Time::from_ms(1500.0), Time::from_ms(2500.0)),
            mock_camera_clip("3", Time::from_ms(3000.0), Time::from_ms(4000.0)),
        ];
        let sequence = mock_sequence(clips);
        let mut editor = mock_editor(sequence);
        // select 3 clip
        editor.select_only_this_clip("3");
        // shift click 2 back
        editor.select_all_to_time(&Time::from_ms(2250.0));
        // result should be 2 and 3.
        assert_eq!(
            editor.get_selected_clip_ids().iter().collect::<Vec<_>>(),
            ["2", "3"]
        );
    }
    #[test]
    #[wasm_bindgen_test]
    fn shift_click_should_select_crossed_middle_clip() {
        // make clip 1, 2, 3
        // clip 1 and 2 is crossed.
        let clips = vec![
            mock_camera_clip("1", Time::from_ms(1000.0), Time::from_ms(2000.0)),
            mock_camera_clip("2", Time::from_ms(1500.0), Time::from_ms(2500.0)),
            mock_camera_clip("3", Time::from_ms(3000.0), Time::from_ms(4000.0)),
        ];
        let sequence = mock_sequence(clips);
        let mut editor = mock_editor(sequence);
        // select 1 clip
        editor.select_only_this_clip("1");
        // shift click 3
        editor.select_all_to_time(&Time::from_ms(3500.0));
        // result should be 1, 2 and 3.
        assert_eq!(
            editor.get_selected_clip_ids().iter().collect::<Vec<_>>(),
            ["1", "2", "3"]
        );
    }
    #[test]
    #[wasm_bindgen_test]
    fn shift_click_should_select_crossed_middle_clip_backward() {
        // make clip 1, 2, 3
        // clip 2 and 3 is crossed.
        let clips = vec![
            mock_camera_clip("1", Time::from_ms(1000.0), Time::from_ms(2000.0)),
            mock_camera_clip("2", Time::from_ms(2000.0), Time::from_ms(3000.0)),
            mock_camera_clip("3", Time::from_ms(2500.0), Time::from_ms(3500.0)),
        ];
        let sequence = mock_sequence(clips);
        let mut editor = mock_editor(sequence);
        // select 3 clip
        editor.select_only_this_clip("3");
        // shift click 1
        editor.select_all_to_time(&Time::from_ms(1500.0));
        // result should be 1, 2 and 3.
        assert_eq!(
            editor.get_selected_clip_ids().iter().collect::<Vec<_>>(),
            ["1", "2", "3"]
        );
    }
    #[test]
    #[wasm_bindgen_test]
    fn shift_click_should_not_select_swallowed_clip_if_not_click_common() {
        // make clip 1, 2, 3
        // clip 2 shallow clip 3.
        let clips = vec![
            mock_camera_clip("1", Time::from_ms(1000.0), Time::from_ms(2000.0)),
            mock_camera_clip("2", Time::from_ms(2000.0), Time::from_ms(4000.0)),
            mock_camera_clip("3", Time::from_ms(3000.0), Time::from_ms(3500.0)),
        ];
        let sequence = mock_sequence(clips);
        let mut editor = mock_editor(sequence);
        // select 1 clip
        editor.select_only_this_clip("1");
        // shift click the parts on 2, not on 3
        editor.select_all_to_time(&Time::from_ms(2500.0));
        // result should be 1 and 2.
        assert_eq!(
            editor.get_selected_clip_ids().iter().collect::<Vec<_>>(),
            ["1", "2"]
        );
    }
    #[test]
    #[wasm_bindgen_test]
    fn shift_click_should_select_swallowed_clip_if_click_common() {
        // make clip 1, 2, 3
        // clip 2 shallow clip 3.
        let clips = vec![
            mock_camera_clip("1", Time::from_ms(1000.0), Time::from_ms(2000.0)),
            mock_camera_clip("2", Time::from_ms(2000.0), Time::from_ms(4000.0)),
            mock_camera_clip("3", Time::from_ms(3000.0), Time::from_ms(3500.0)),
        ];
        let sequence = mock_sequence(clips);
        let mut editor = mock_editor(sequence);
        // select 1 clip
        editor.select_only_this_clip("1");
        // shift click the parts on 2 and 3
        editor.select_all_to_time(&Time::from_ms(3250.0));
        // result should be 1, 2 and 3.
        assert_eq!(
            editor.get_selected_clip_ids().iter().collect::<Vec<_>>(),
            ["1", "2", "3"]
        );
    }

    fn mock_socket() -> Socket {
        let (sender, receiver) = tokio::sync::mpsc::unbounded_channel();
        let response_waiter = ResponseWaiter::new();
        Socket::new(sender.clone(), response_waiter.clone())
    }
    fn mock_sequence(camera_clips: Vec<Arc<CameraClip>>) -> Arc<Sequence> {
        Arc::new(Sequence {
            tracks: vec![
                Arc::new(Track::Camera(CameraTrack {
                    id: "track-1".to_string(),
                    clips: camera_clips.into(),
                })),
                Arc::new(Track::Subtitle(SubtitleTrack {
                    id: "track-2".to_string(),
                    clips: Arc::new([]),
                })),
            ]
            .into(),
        })
    }
    fn mock_editor(sequence: Arc<Sequence>) -> Editor {
        let socket = mock_socket();
        Editor {
            timeline: Timeline::new(),
            image_filename_objects: vec![],
            job: None,
            clip_editor: None,
            selected_clip_ids: Arc::new(BTreeSet::new()),
            sequence_player: Box::new(MockSequencePlay::new()),
            history: History::new(sequence.clone()),
            top_bar: TopBar::new(),
            clipboard: None,
            language: namui::Language::Ko,
            clip_id_to_check_as_click: None,
            context_menu: None,
            sequence_saver: SequenceSaver::new("", sequence.clone(), socket.clone()),
            sheet_sequence_syncer: SheetSequenceSyncer::new(""),
            meta_container: Arc::new(MetaContainer::new(
                Some(Meta {
                    subtitle_language_minimum_play_duration_map: HashMap::<_, _>::from_iter(
                        IntoIter::new([(Language::Ko, Time::from_ms(1000.0))]),
                    ),
                    subtitle_language_play_duration_per_character_map: HashMap::<_, _>::from_iter(
                        IntoIter::new([(Language::Ko, Time::from_ms(100.0))]),
                    ),
                }),
                Arc::new(MockMetaLoad::new()),
            )),
        }
    }
}
