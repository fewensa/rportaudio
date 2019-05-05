#[macro_use]
extern crate bitflags;
extern crate libc;


//pub(self) use crate::pa_include::portaudio::root as raw_portaudio;
pub(self) use portaudio_sys as raw_portaudio;

pub(self) mod pa_include;

pub mod types;

mod rpa_error;
mod rpa_linux_alsa;
mod rpa_ringbuffer;
mod rpa_util;
mod rportaudio;
mod kit;
