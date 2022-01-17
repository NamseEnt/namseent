use self::content_loader::ContentLoader;
use crate::app::types::*;
use namui::RenderingTree;
use std::{rc::Rc, time::Duration};
mod content_loader;

pub struct SequencePlayer {
    id: String,
    is_playing: bool,
    sequence: Rc<Sequence>,
    content_loader: ContentLoader,
}

enum SequencePlayerEvent {
    CheckLoading(String),
}

pub struct SequencePlayerProps {}

impl SequencePlayer {
    pub fn new(
        sequence: Rc<Sequence>,
        camera_angle_image_loader: &dyn CameraAngleImageLoader,
    ) -> Self {
        let id = namui::nanoid();
        let this = Self {
            id: id.clone(),
            is_playing: false,
            sequence: sequence.clone(),
            content_loader: ContentLoader::new(sequence.clone(), camera_angle_image_loader),
        };
        this.call_loading_timeout();
        this
    }
    pub fn play(&mut self) {
        self.is_playing = true;
    }
    pub fn stop(&mut self) {
        self.is_playing = false;
    }
    pub fn update_sequence(
        &mut self,
        sequence: Rc<Sequence>,
        camera_angle_image_loader: &dyn CameraAngleImageLoader,
    ) {
        self.sequence = sequence.clone();
        self.content_loader = ContentLoader::new(sequence, camera_angle_image_loader);
    }
    pub fn update(&mut self, event: &dyn std::any::Any) {
        if let Some(event) = event.downcast_ref::<SequencePlayerEvent>() {
            match event {
                SequencePlayerEvent::CheckLoading(id) => {
                    if id.ne(&self.id) {
                        return;
                    }
                    match self.content_loader.is_loaded() {
                        false => {
                            namui::log!("SequencePlayer::update: loading not yet");
                            self.call_loading_timeout()
                        }
                        true => {
                            namui::log!("SequencePlayer::update: loaded");
                        }
                    }
                }
            }
        }
    }
    pub fn render(&self, props: &SequencePlayerProps) -> RenderingTree {
        // TODO
        RenderingTree::Empty
    }
    fn call_loading_timeout(&self) {
        let id = self.id.clone();
        namui::set_timeout(
            move || namui::event::send(SequencePlayerEvent::CheckLoading(id)),
            Duration::from_secs(1),
        );
    }
}
