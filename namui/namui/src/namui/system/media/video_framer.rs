use super::{media_control::MediaControlReceiver, WithInstant, VIDEO_CHANNEL_BOUND};
use namui_type::*;
use std::{collections::VecDeque, ops::Deref};

#[derive(Debug)]
pub(crate) struct VideoFramer {
    control_receiver: MediaControlReceiver,
    images: VecDeque<WithInstant<ImageHandle>>,
    image_handle_rx: std::sync::mpsc::Receiver<WithInstant<ImageHandle>>,
    frame_received_after_start: usize,
    fps: f64,
    last_start_requested: Option<Instant>,
}

impl VideoFramer {
    pub(crate) fn new(
        image_handle_rx: std::sync::mpsc::Receiver<WithInstant<ImageHandle>>,
        control_receiver: MediaControlReceiver,
        fps: f64,
    ) -> VideoFramer {
        Self {
            control_receiver,
            images: VecDeque::new(),
            image_handle_rx,
            frame_received_after_start: 0,
            fps,
            last_start_requested: None,
        }
    }

    pub(crate) fn get_image(&mut self) -> Option<namui_type::ImageHandle> {
        self.fill_images();

        let Some(start_requested) = self.control_receiver.start_requested() else {
            return self.images.front().map(|frame| frame.deref().clone());
        };

        if self.last_start_requested != Some(start_requested) {
            self.frame_received_after_start = 0;
            self.last_start_requested = Some(start_requested);
        }

        let expected_frame_index =
            ((crate::time::now() - start_requested).as_secs_f64() * self.fps) as usize;

        while self.frame_received_after_start < expected_frame_index {
            if self.images.pop_front().is_some() {
                self.frame_received_after_start += 1;
            } else {
                break;
            }
        }

        self.images.front().map(|frame| frame.deref().clone())
    }

    fn fill_images(&mut self) {
        let flush_requested = self.control_receiver.flush_requested();

        if let Some(flush_requested) = flush_requested {
            self.images.retain(|chunk| flush_requested < chunk.instant);
        }

        while self.images.len() < VIDEO_CHANNEL_BOUND {
            let Ok(chunk) = self.image_handle_rx.try_recv() else {
                break;
            };

            if let Some(flush_requested) = flush_requested {
                if chunk.instant < flush_requested {
                    continue;
                }
            }

            self.images.push_back(chunk);
        }
    }
}
