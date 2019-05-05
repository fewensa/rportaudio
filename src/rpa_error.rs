use std::fmt;

use crate::raw_portaudio;


#[repr(i32)]
#[derive(PartialEq, Copy, Clone)]
#[allow(missing_docs)]
pub enum PaError {
  PaNoError = raw_portaudio::paNoError,
  PaNotInitialized = raw_portaudio::paNotInitialized,
  PaUnanticipatedHostError = raw_portaudio::paUnanticipatedHostError,
  PaInvalidChannelCount = raw_portaudio::paInvalidChannelCount,
  PaInvalidSampleRate = raw_portaudio::paInvalidSampleRate,
  PaInvalidDevice = raw_portaudio::paInvalidDevice,
  PaInvalidFlag = raw_portaudio::paInvalidFlag,
  PaSampleFormatNotSupported = raw_portaudio::paSampleFormatNotSupported,
  PaBadIODeviceCombination = raw_portaudio::paBadIODeviceCombination,
  PaInsufficientMemory = raw_portaudio::paInsufficientMemory,
  PaBufferTooBig = raw_portaudio::paBufferTooBig,
  PaBufferTooSmall = raw_portaudio::paBufferTooSmall,
  PaNullCallback = raw_portaudio::paNullCallback,
  PaBadStreamPtr = raw_portaudio::paBadStreamPtr,
  PaTimedOut = raw_portaudio::paTimedOut,
  PaInternalError = raw_portaudio::paInternalError,
  PaDeviceUnavailable = raw_portaudio::paDeviceUnavailable,
  PaIncompatibleHostApiSpecificStreamInfo = raw_portaudio::paIncompatibleHostApiSpecificStreamInfo,
  PaStreamIsStopped = raw_portaudio::paStreamIsStopped,
  PaStreamIsNotStopped = raw_portaudio::paStreamIsNotStopped,
  PaInputOverflowed = raw_portaudio::paInputOverflowed,
  PaOutputUnderflowed = raw_portaudio::paOutputUnderflowed,
  PaHostApiNotFound = raw_portaudio::paHostApiNotFound,
  PaInvalidHostApi = raw_portaudio::paInvalidHostApi,
  PaCanNotReadFromACallbackStream = raw_portaudio::paCanNotReadFromACallbackStream,
  PaCanNotWriteToACallbackStream = raw_portaudio::paCanNotWriteToACallbackStream,
  PaCanNotReadFromAnOutputOnlyStream = raw_portaudio::paCanNotReadFromAnOutputOnlyStream,
  PaCanNotWriteToAnInputOnlyStream = raw_portaudio::paCanNotWriteToAnInputOnlyStream,
  PaIncompatibleStreamHostApi = raw_portaudio::paIncompatibleStreamHostApi,
  PaBadBufferPtr = raw_portaudio::paBadBufferPtr,
  UnknownError,
}


impl PaError {
  /// Get the enum value corresponding to the given i32
  pub fn from_i32(num: i32) -> PaError {
    match num {
      raw_portaudio::paNoError => PaError::PaNoError,
      raw_portaudio::paNotInitialized => PaError::PaNotInitialized,
      raw_portaudio::paUnanticipatedHostError => PaError::PaUnanticipatedHostError,
      raw_portaudio::paInvalidChannelCount => PaError::PaInvalidChannelCount,
      raw_portaudio::paInvalidSampleRate => PaError::PaInvalidSampleRate,
      raw_portaudio::paInvalidDevice => PaError::PaInvalidDevice,
      raw_portaudio::paInvalidFlag => PaError::PaInvalidFlag,
      raw_portaudio::paSampleFormatNotSupported => PaError::PaSampleFormatNotSupported,
      raw_portaudio::paBadIODeviceCombination => PaError::PaBadIODeviceCombination,
      raw_portaudio::paInsufficientMemory => PaError::PaInsufficientMemory,
      raw_portaudio::paBufferTooBig => PaError::PaBufferTooBig,
      raw_portaudio::paBufferTooSmall => PaError::PaBufferTooSmall,
      raw_portaudio::paNullCallback => PaError::PaNullCallback,
      raw_portaudio::paBadStreamPtr => PaError::PaBadStreamPtr,
      raw_portaudio::paTimedOut => PaError::PaTimedOut,
      raw_portaudio::paInternalError => PaError::PaInternalError,
      raw_portaudio::paDeviceUnavailable => PaError::PaDeviceUnavailable,
      raw_portaudio::paIncompatibleHostApiSpecificStreamInfo => PaError::PaIncompatibleHostApiSpecificStreamInfo,
      raw_portaudio::paStreamIsStopped => PaError::PaStreamIsStopped,
      raw_portaudio::paStreamIsNotStopped => PaError::PaStreamIsNotStopped,
      raw_portaudio::paInputOverflowed => PaError::PaInputOverflowed,
      raw_portaudio::paOutputUnderflowed => PaError::PaOutputUnderflowed,
      raw_portaudio::paHostApiNotFound => PaError::PaHostApiNotFound,
      raw_portaudio::paInvalidHostApi => PaError::PaInvalidHostApi,
      raw_portaudio::paCanNotReadFromACallbackStream => PaError::PaCanNotReadFromACallbackStream,
      raw_portaudio::paCanNotWriteToACallbackStream => PaError::PaCanNotWriteToACallbackStream,
      raw_portaudio::paCanNotReadFromAnOutputOnlyStream => PaError::PaCanNotReadFromAnOutputOnlyStream,
      raw_portaudio::paCanNotWriteToAnInputOnlyStream => PaError::PaCanNotWriteToAnInputOnlyStream,
      raw_portaudio::paIncompatibleStreamHostApi => PaError::PaIncompatibleStreamHostApi,
      raw_portaudio::paBadBufferPtr => PaError::PaBadBufferPtr,
      _ => PaError::UnknownError,
    }
  }
}

impl fmt::Display for PaError {
  fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
//    match *self {
//      PaError::UnknownError => write!(f, "Unknown Error"),
//      other => {
//        let message_c = unsafe { raw_portaudio::Pa_GetErrorText(other as i32) };
//        let message_s = String::from_utf8_lossy(unsafe { CStr::from_ptr(message_c).to_bytes() });
//        f.write_str(&*message_s)
//      }
//    }
    let msg = crate::rportaudio::error_text(*self);
    f.write_str(&msg[..])
  }
}

impl fmt::Debug for PaError {
  fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
    ::std::fmt::Display::fmt(self, fmt)
  }
}

/// A result type wrapping PaError.
///
/// The original NoError is mapped to Ok(()) and other values mapped to Err(x)
pub type PaResult = Result<(), PaError>;

