use super::flush_button::FlushButtonReceiver;

#[derive(Debug)]
pub(crate) struct AudioBuffer {
    consumer: rtrb::Consumer<Vec<f32>>,
    index: usize,
    flush_requested: FlushButtonReceiver,
}

impl AudioBuffer {
    pub(crate) fn new(
        consumer: rtrb::Consumer<Vec<f32>>,
        flush_requested: FlushButtonReceiver,
    ) -> Self {
        Self {
            consumer,
            index: 0,
            flush_requested,
        }
    }
    pub(crate) fn consume(&mut self, output: &mut [f32]) {
        self.flush_if_requested();

        let mut output_index = 0;

        while output_index < output.len() {
            let Ok(chunk) = self.consumer.peek() else {
                break;
            };

            let left = self.index;
            let right = chunk.len().min(output.len() - output_index);
            let chunk_to_copy = &chunk[left..right];

            for (i, v) in chunk_to_copy.iter().enumerate() {
                output[output_index + i] += *v;
            }

            output_index += chunk_to_copy.len();
            if right == chunk.len() {
                self.consumer.pop().unwrap();
                self.index = 0;
            } else {
                self.index = right;
                assert_eq!(output_index, output.len());
                break;
            }
        }
    }

    pub(crate) fn is_end(&self) -> bool {
        self.consumer.is_abandoned() && self.consumer.is_empty()
    }

    fn flush_if_requested(&mut self) {
        if !self.flush_requested.take() {
            return;
        }

        self.consumer
            .read_chunk(self.consumer.slots())
            .unwrap()
            .commit_all();
        self.index = 0;
    }
}
