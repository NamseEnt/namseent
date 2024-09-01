use super::input_context::InputContext;
use crate::system::media::{
    core::{DecodingStream, MediaController},
    with_instant::WithInstant,
    VIDEO_CHANNEL_BOUND,
};
use anyhow::Result;
use namui_type::*;

#[derive(Debug)]
pub(crate) enum DecodingThreadCommand {
    Play,
    Stop,
    Pause,
    SeekTo {
        duration: Duration,
    },
    WaitForPreload {
        finish_tx: tokio::sync::oneshot::Sender<()>,
    },
}

pub(crate) fn spawn_decoding_thread(
    input_ctx: InputContext,
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
            wait_for_preload_finish_tx: None,
        }
        .run()
        .unwrap()
    });
}

struct DecodingThreadRunner {
    input_ctx: InputContext,
    decoding_streams: Vec<Option<DecodingStream>>,
    command_rx: std::sync::mpsc::Receiver<WithInstant<DecodingThreadCommand>>,
    media_controller: MediaController,
    eof: bool,
    is_playing: bool,
    wait_for_preload_finish_tx: Option<tokio::sync::oneshot::Sender<()>>,
}

impl DecodingThreadRunner {
    fn run(mut self) -> Result<()> {
        let mut is_command_disconnected = false;

        loop {
            while !is_command_disconnected {
                match self.command_rx.try_recv() {
                    Ok(command) => {
                        self.handle_command(command.inner())?;
                    }
                    Err(err) => {
                        if let std::sync::mpsc::TryRecvError::Disconnected = err {
                            is_command_disconnected = true;
                        }
                        break;
                    }
                }
            }

            let is_abandoned = is_command_disconnected && (self.eof || !self.is_playing);

            if is_abandoned {
                return Ok(());
            }

            let should_push_packet = !self.eof && (self.is_playing || !self.is_preload_finished());

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
            if self.wait_for_preload_finish_tx.is_some() && self.is_preload_finished() {
                let _ = self.wait_for_preload_finish_tx.take().unwrap().send(());
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

                self.wait_for_preload_finish_tx = None;
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

                self.iter_mut_decoding_streams()
                    .for_each(|decoding_stream| decoding_stream.seek_to(duration));

                self.wait_for_preload_finish_tx = None;
            }
            DecodingThreadCommand::WaitForPreload { finish_tx } => {
                self.wait_for_preload_finish_tx = Some(finish_tx);
            }
        }

        Ok(())
    }

    fn iter_decoding_streams(&self) -> impl Iterator<Item = &DecodingStream> {
        self.decoding_streams
            .iter()
            .filter_map(|decoding_stream| decoding_stream.as_ref())
    }

    fn iter_mut_decoding_streams(&mut self) -> impl Iterator<Item = &mut DecodingStream> {
        self.decoding_streams
            .iter_mut()
            .filter_map(|decoding_stream| decoding_stream.as_mut())
    }

    fn reset_decoders(&mut self) -> Result<()> {
        self.iter_mut_decoding_streams()
            .try_for_each(|decoding_stream| decoding_stream.reset_decoder())?;
        Ok(())
    }

    fn send_eof(&mut self) -> Result<()> {
        self.iter_mut_decoding_streams()
            .try_for_each(|decoding_stream| -> Result<()> {
                decoding_stream.send_eof()?;
                decoding_stream.receive_and_process_decoded_frames()?;
                Ok(())
            })?;
        Ok(())
    }

    fn is_preload_finished(&self) -> bool {
        self.iter_decoding_streams()
            .map(|stream| stream.sent_count())
            .all(|x| x >= VIDEO_CHANNEL_BOUND)
    }
}
