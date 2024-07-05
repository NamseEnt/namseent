use super::Stream;
use ffi::*;
use format::context::common::Context;
use std::ops::Deref;
use {codec, Dictionary, Rational};

pub struct StreamMut {
    context: Context,
    index: usize,

    immutable: Stream,
}

impl StreamMut {
    pub unsafe fn wrap(context: Context, index: usize) -> StreamMut {
        StreamMut {
            context: context.clone(),
            index,
            immutable: Stream::wrap(context, index),
        }
    }

    pub unsafe fn as_mut_ptr(&mut self) -> *mut AVStream {
        *(*self.context.as_mut_ptr()).streams.add(self.index)
    }
}

impl StreamMut {
    pub fn set_time_base<R: Into<Rational>>(&mut self, value: R) {
        unsafe {
            (*self.as_mut_ptr()).time_base = value.into().into();
        }
    }

    pub fn set_rate<R: Into<Rational>>(&mut self, value: R) {
        unsafe {
            (*self.as_mut_ptr()).r_frame_rate = value.into().into();
        }
    }

    pub fn set_avg_frame_rate<R: Into<Rational>>(&mut self, value: R) {
        unsafe {
            (*self.as_mut_ptr()).avg_frame_rate = value.into().into();
        }
    }

    pub fn set_parameters<P: Into<codec::Parameters>>(&mut self, parameters: P) {
        let parameters = parameters.into();

        unsafe {
            avcodec_parameters_copy((*self.as_mut_ptr()).codecpar, parameters.as_ptr());
        }
    }

    pub fn set_metadata(&mut self, metadata: Dictionary) {
        unsafe {
            let metadata = metadata.disown();
            (*self.as_mut_ptr()).metadata = metadata;
        }
    }
}

impl Deref for StreamMut {
    type Target = Stream;

    fn deref(&self) -> &Self::Target {
        &self.immutable
    }
}
