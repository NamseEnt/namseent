use crate::system::media::{
    core::MediaControlReceiver, with_instant::WithInstant, AUDIO_CHANNEL_BOUND,
};
use std::{collections::VecDeque, sync::mpsc::TryRecvError};

#[derive(Debug)]
pub(crate) struct AudioBuffer {
    rx: std::sync::mpsc::Receiver<WithInstant<Vec<f32>>>,
    index: usize,
    control_receiver: MediaControlReceiver,
    chunks: VecDeque<WithInstant<Vec<f32>>>,
}

impl AudioBuffer {
    pub(crate) fn new(
        rx: std::sync::mpsc::Receiver<WithInstant<Vec<f32>>>,
        control_receiver: MediaControlReceiver,
    ) -> Self {
        Self {
            rx,
            index: 0,
            control_receiver,
            chunks: VecDeque::new(),
        }
    }
    pub(crate) fn consume(&mut self, output: &mut [f32]) {
        self.fill_chunks();

        if self.control_receiver.start_requested().is_none() {
            // TODO: Maybe need to sync with start_requested Instant?
            return;
        }

        let mut output_index = 0;

        while output_index < output.len() {
            let Some(chunk) = self.chunks.front() else {
                break;
            };

            let to_fill_count = output.len() - output_index;

            let left = self.index;
            let right = chunk.len().min(left + to_fill_count);
            let chunk_to_copy = &chunk[left..right];

            for (i, v) in chunk_to_copy.iter().enumerate() {
                output[output_index + i] += *v;
            }

            output_index += chunk_to_copy.len();
            if right == chunk.len() {
                self.chunks.pop_front();
                self.index = 0;
            } else {
                self.index = right;
                assert_eq!(output_index, output.len());
                break;
            }
        }
    }

    pub(crate) fn is_end(&self) -> bool {
        self.chunks.is_empty()
            && self
                .rx
                .try_recv()
                .is_err_and(|e| matches!(e, TryRecvError::Disconnected))
    }

    fn fill_chunks(&mut self) {
        let flush_requested = self.control_receiver.flush_requested();

        if let Some(flush_requested) = flush_requested {
            self.chunks.retain(|chunk| flush_requested < chunk.instant);
        }

        while self.chunks.len() < AUDIO_CHANNEL_BOUND {
            let Ok(chunk) = self.rx.try_recv() else {
                break;
            };

            if let Some(flush_requested) = flush_requested {
                if chunk.instant < flush_requested {
                    continue;
                }
            }

            self.chunks.push_back(chunk);
        }
    }
}
