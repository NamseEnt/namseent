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
}

impl DecodingThreadRunner {
    fn run(mut self) -> Result<()> {
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

            let should_push_packet = !self.eof
                && (self.is_playing
                    || self
                        .map_decoding_streams(|stream| stream.sent_count())?
                        .into_iter()
                        .any(|x| x < VIDEO_CHANNEL_BOUND));

            if !should_push_packet {
                std::thread::yield_now();
                continue;
            }

            match self.input_ctx.packets().next() {
                Some((stream, packet)) => {
                    let Some(decoding_stream) = &mut self.decoding_streams[stream.index()] else {
                        continue;
                    };

                    decoding_stream.send_packet(&packet)?;
                    decoding_stream.receive_and_process_decoded_frames()?;
                }
                None => {
                    if self.eof {
                        continue;
                    }

                    self.eof = true;

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

                self.media_controller.flush();
                self.media_controller.stop();

                self.input_ctx.seek(0, ..)?;

                self.eof = false;

                self.reset_decoders()?;
            }
            DecodingThreadCommand::Pause => {
                self.is_playing = false;
                self.media_controller.stop();
            }
            DecodingThreadCommand::SeekTo { duration } => {
                self.media_controller.flush();

                let micros = duration.as_micros() as i64;
                self.input_ctx.seek(micros, ..)?;

                self.eof = false;
                self.reset_decoders()?;

                self.map_decoding_streams(|decoding_stream| decoding_stream.seek_to(duration))?;
            }
        }

        Ok(())
    }

    fn map_decoding_streams<T>(
        &mut self,
        callback: impl Fn(&mut DecodingStream) -> Result<T>,
    ) -> Result<Vec<T>> {
        self.decoding_streams
            .iter_mut()
            .filter_map(|decoding_stream| decoding_stream.as_mut())
            .map(callback)
            .collect::<Result<Vec<_>>>()
    }

    fn reset_decoders(&mut self) -> Result<()> {
        self.map_decoding_streams(|decoding_stream| decoding_stream.reset_decoder())?;
        Ok(())
    }

    fn send_eof(&mut self) -> Result<()> {
        self.map_decoding_streams(|decoding_stream| {
            decoding_stream.send_eof()?;
            decoding_stream.receive_and_process_decoded_frames()?;
            Ok(())
        })?;
        Ok(())
    }
}
