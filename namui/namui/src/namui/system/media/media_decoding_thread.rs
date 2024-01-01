use super::{
    media_control::MediaController, media_decoding_stream::DecodingStream,
    with_instant::WithInstant, VIDEO_CHANNEL_BOUND,
};
use anyhow::Result;
use namui_type::*;

#[derive(Debug)]
pub(crate) enum DecodingThreadCommand {
    Play,
    Stop,
    Pause,
    SeekTo { duration: Duration },
}

pub(crate) fn spawn_decoding_thread(
    input_ctx: ffmpeg_next::format::context::Input,
    decoding_streams: Vec<Option<DecodingStream>>,
    command_rx: std::sync::mpsc::Receiver<WithInstant<DecodingThreadCommand>>,
    media_controller: MediaController,
) {
    std::thread::spawn(move || {
        DecodingThreadRunner {
            input_ctx,
            decoding_streams,
            command_rx,
            media_controller,
            eof: false,
            is_playing: false,
            preload_count: 0,
        }
        .run()
        .unwrap()
    });
}

struct DecodingThreadRunner {
    input_ctx: ffmpeg_next::format::context::Input,
    decoding_streams: Vec<Option<DecodingStream>>,
    command_rx: std::sync::mpsc::Receiver<WithInstant<DecodingThreadCommand>>,
    media_controller: MediaController,
    eof: bool,
    is_playing: bool,
    preload_count: usize,
}

impl DecodingThreadRunner {
    fn run(mut self) -> Result<()> {
        let mut index = 0;
        loop {
            loop {
                match self.command_rx.try_recv() {
                    Ok(command) => {
                        self.handle_command(command.inner())?;
                    }
                    Err(err) => match err {
                        std::sync::mpsc::TryRecvError::Empty => {
                            break;
                        }
                        std::sync::mpsc::TryRecvError::Disconnected => {
                            println!("command disconnected");
                            return Ok(());
                        }
                    },
                }
            }

            if !self.is_playing && (self.eof || self.preload_count >= VIDEO_CHANNEL_BOUND) {
                std::thread::yield_now();
                continue;
            }

            match self.input_ctx.packets().next() {
                Some((stream, packet)) => {
                    let Some(decoding_stream) = &mut self.decoding_streams[stream.index()] else {
                        continue;
                    };

                    if !self.is_playing {
                        println!("preload: {}", self.preload_count);
                        self.preload_count += 1;
                    }

                    println!("index: {index}");
                    index += 1;

                    decoding_stream.send_packet(&packet)?;
                    decoding_stream.receive_and_process_decoded_frames()?;
                }
                None => {
                    if self.eof {
                        continue;
                    }

                    self.eof = true;
                    println!("EOF");

                    self.send_eof()?;
                }
            }
        }
    }
    fn handle_command(&mut self, command: DecodingThreadCommand) -> Result<()> {
        match command {
            DecodingThreadCommand::Play => {
                self.is_playing = true;
                self.media_controller.start();
            }
            DecodingThreadCommand::Stop => {
                self.is_playing = false;
                self.preload_count = 0;
                self.media_controller.flush();
                self.media_controller.stop();
                self.input_ctx.seek(0, ..)?;
                self.eof = false;
                self.reset_decoders()?;
            }
            DecodingThreadCommand::Pause => {
                self.is_playing = false;
                self.preload_count = 0;
                self.media_controller.stop();
            }
            DecodingThreadCommand::SeekTo { duration } => {
                println!("Seek to: {:?}", duration);
                self.media_controller.flush();
                self.preload_count = 0;

                let micros = duration.as_micros() as i64;
                self.input_ctx.seek(micros, ..micros)?;

                self.eof = false;
                self.reset_decoders()?;
            }
        }

        Ok(())
    }

    fn reset_decoders(&mut self) -> Result<()> {
        self.decoding_streams
            .iter_mut()
            .filter_map(|decoding_stream| decoding_stream.as_mut())
            .map(|decoding_stream| decoding_stream.reset_decoder())
            .collect::<Result<Vec<_>>>()?;
        Ok(())
    }

    fn send_eof(&mut self) -> Result<()> {
        self.decoding_streams
            .iter_mut()
            .filter_map(|decoding_stream| decoding_stream.as_mut())
            .map(|decoding_stream| -> Result<_> {
                decoding_stream.send_eof()?;
                decoding_stream.receive_and_process_decoded_frames()?;
                Ok(())
            })
            .collect::<Result<Vec<_>>>()?;

        Ok(())
    }
}
