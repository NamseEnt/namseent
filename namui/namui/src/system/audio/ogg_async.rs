//! Reimplement of https://github.com/RustAudio/ogg/blob/4b22c17d4ac365e8f4e16c69f0e906a95731e781/src/reading.rs#L1055-L1211

use super::*;
use bytes::BytesMut;
use futures::{Stream, ready};
use ogg::{Packet, reading::*};
use std::pin::Pin;
use std::task::{Context, Poll};

enum PageDecodeState {
    Head,
    Segments(PageParser, usize),
    PacketData(PageParser, usize),
    InUpdate,
}

impl PageDecodeState {
    fn needed_size(&self) -> usize {
        match self {
            PageDecodeState::Head => 27,
            PageDecodeState::Segments(_, s) => *s,
            PageDecodeState::PacketData(_, s) => *s,
            PageDecodeState::InUpdate => panic!("invalid state"),
        }
    }
}

/**
Async page reading functionality.
*/
pub struct PageDecoder {
    state: PageDecodeState,
}

impl PageDecoder {
    fn new() -> Self {
        PageDecoder {
            state: PageDecodeState::Head,
        }
    }
}

impl PageDecoder {
    fn decode(&mut self, buf: &mut BytesMut) -> Result<Option<OggPage>, OggReadError> {
        use self::PageDecodeState::*;
        loop {
            let needed_size = self.state.needed_size();
            if buf.len() < needed_size {
                return Ok(None);
            }
            let mut ret = None;
            let consumed_buf = buf.split_to(needed_size).to_vec();

            self.state = match ::std::mem::replace(&mut self.state, InUpdate) {
                Head => {
                    let mut hdr_buf = [0; 27];
                    // TODO once we have const generics, the copy below can be done
                    // much nicer, maybe with a new into_array fn on Vec's
                    hdr_buf.copy_from_slice(&consumed_buf);
                    let tup = PageParser::new(hdr_buf)?;
                    Segments(tup.0, tup.1)
                }
                Segments(mut pg_prs, _) => {
                    let new_needed_len = pg_prs.parse_segments(consumed_buf);
                    PacketData(pg_prs, new_needed_len)
                }
                PacketData(pg_prs, _) => {
                    ret = Some((pg_prs.parse_packet_data(consumed_buf))?);
                    Head
                }
                InUpdate => panic!("invalid state"),
            };
            if ret.is_some() {
                return Ok(ret);
            }
        }
    }
}

#[pin_project::pin_project]
pub struct PacketReader<T, Error>
where
    T: futures::TryStream<Ok = bytes::Bytes, Error = Error>,
{
    base_packet_reader: BasePacketReader,
    #[pin]
    page_read: FramedRead<T, Error>,
}

impl<T, Error> PacketReader<T, Error>
where
    T: futures::TryStream<Ok = bytes::Bytes, Error = Error>,
{
    pub fn new(stream: T) -> Self {
        PacketReader {
            base_packet_reader: BasePacketReader::new(),
            page_read: FramedRead::new(stream, PageDecoder::new()),
        }
    }
}

#[pin_project::pin_project]
struct FramedRead<T, Error>
where
    T: futures::TryStream<Ok = bytes::Bytes, Error = Error>,
{
    #[pin]
    stream: T,
    decoder: PageDecoder,
    bytes_mut: BytesMut,
    _error: std::marker::PhantomData<Error>,
}

impl<T, Error> FramedRead<T, Error>
where
    T: futures::TryStream<Ok = bytes::Bytes, Error = Error>,
{
    fn new(stream: T, decoder: PageDecoder) -> Self {
        FramedRead {
            stream,
            decoder,
            bytes_mut: BytesMut::new(),
            _error: std::marker::PhantomData,
        }
    }
}

impl<T, Error> Stream for FramedRead<T, Error>
where
    T: futures::TryStream<Ok = bytes::Bytes, Error = Error>,
{
    type Item = Result<OggPage, OggAsyncReadError<Error>>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let mut this = self.project();
        loop {
            let mut eof = false;
            match ready!(this.stream.as_mut().try_poll_next(cx)) {
                Some(Ok(buf)) => this.bytes_mut.extend_from_slice(&buf),
                Some(Err(err)) => return Poll::Ready(Some(Err(OggAsyncReadError::Stream(err)))),
                None => eof = true,
            }
            match this.decoder.decode(this.bytes_mut) {
                Ok(Some(page)) => return Poll::Ready(Some(Ok(page))),
                Ok(None) => {}
                Err(err) => return Poll::Ready(Some(Err(OggAsyncReadError::Ogg(err)))),
            }

            if eof {
                return Poll::Ready(None);
            }
        }
    }
}

impl<T, Error> Stream for PacketReader<T, Error>
where
    T: futures::TryStream<Ok = bytes::Bytes, Error = Error>,
{
    type Item = Result<Packet, OggAsyncReadError<Error>>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let mut this = self.project();
        // Read pages until we got a valid entire packet
        // (packets may span multiple pages, so reading one page
        // doesn't always suffice to give us a valid packet)
        loop {
            if let Some(pck) = this.base_packet_reader.read_packet() {
                return Poll::Ready(Some(Ok(pck)));
            }
            let page = match ready!(this.page_read.as_mut().poll_next(cx)) {
                Some(Ok(page)) => page,
                Some(Err(err)) => return Poll::Ready(Some(Err(err))),
                None => return Poll::Ready(None),
            };
            match this.base_packet_reader.push_page(page) {
                Ok(_) => {}
                Err(err) => return Poll::Ready(Some(Err(OggAsyncReadError::Ogg(err)))),
            };
        }
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum OggAsyncReadError<Error> {
    Ogg(OggReadError),
    Stream(Error),
}

impl<Error: std::error::Error> std::fmt::Display for OggAsyncReadError<Error> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
impl<Error: std::error::Error> std::error::Error for OggAsyncReadError<Error> {}
