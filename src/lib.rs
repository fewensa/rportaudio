#[macro_use]
extern crate bitflags;
extern crate libc;

pub use self::rportaudio::{error_text, initialize, terminate, version, version_text, hostapi_device_index_to_device_index};

use crate::pa_include::portaudio as raw_portaudio;


pub mod types;
pub mod rpa_error;
pub mod device;
pub mod hostapi;
pub mod stream;

mod pa_include;
mod rportaudio;
mod kit;
