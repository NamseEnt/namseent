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
    pub(super) fn select_all_between_clips<T: AsRef<str>>(&mut self, clip_ids: &[T]) {
        let track = self.get_selected_clip_track().unwrap();

        let clips = clip_ids
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

        for clip in track.get_clips() {
            if clip.get_start_time() >= most_left_clip.get_start_time()
                && self.get_clip_end_time(&clip) <= self.get_clip_end_time(most_right_clip)
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
