#[macro_use]
extern crate bitflags;
extern crate libc;

//use crate::pa_include::portaudio::root as raw_portaudio;
use portaudio_sys as raw_portaudio;
use crate::pa_include::pa_ringbuffer::root as raw_pa_ringbuffer;
use crate::pa_include::pa_util::root as raw_pa_util;


pub use rportaudio::*;


pub mod types;
pub mod rpa_util;
pub mod rpa_ringbuffer;
pub mod rpa_error;

mod pa_include;
mod rportaudio;
mod rpa_linux_alsa;
mod kit;
