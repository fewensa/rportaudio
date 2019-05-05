use std::fmt;

use crate::raw_portaudio;


#[repr(i32)]
#[derive(PartialEq, Copy, Clone)]
#[allow(missing_docs)]
pub enum PaError {
  PaNoError = raw_portaudio::PaErrorCode_paNoError,
  PaNotInitialized = raw_portaudio::PaErrorCode_paNotInitialized,
  PaUnanticipatedHostError = raw_portaudio::PaErrorCode_paUnanticipatedHostError,
  PaInvalidChannelCount = raw_portaudio::PaErrorCode_paInvalidChannelCount,
  PaInvalidSampleRate = raw_portaudio::PaErrorCode_paInvalidSampleRate,
  PaInvalidDevice = raw_portaudio::PaErrorCode_paInvalidDevice,
  PaInvalidFlag = raw_portaudio::PaErrorCode_paInvalidFlag,
  PaSampleFormatNotSupported = raw_portaudio::PaErrorCode_paSampleFormatNotSupported,
  PaBadIODeviceCombination = raw_portaudio::PaErrorCode_paBadIODeviceCombination,
  PaInsufficientMemory = raw_portaudio::PaErrorCode_paInsufficientMemory,
  PaBufferTooBig = raw_portaudio::PaErrorCode_paBufferTooBig,
  PaBufferTooSmall = raw_portaudio::PaErrorCode_paBufferTooSmall,
  PaNullCallback = raw_portaudio::PaErrorCode_paNullCallback,
  PaBadStreamPtr = raw_portaudio::PaErrorCode_paBadStreamPtr,
  PaTimedOut = raw_portaudio::PaErrorCode_paTimedOut,
  PaInternalError = raw_portaudio::PaErrorCode_paInternalError,
  PaDeviceUnavailable = raw_portaudio::PaErrorCode_paDeviceUnavailable,
  PaIncompatibleHostApiSpecificStreamInfo = raw_portaudio::PaErrorCode_paIncompatibleHostApiSpecificStreamInfo,
  PaStreamIsStopped = raw_portaudio::PaErrorCode_paStreamIsStopped,
  PaStreamIsNotStopped = raw_portaudio::PaErrorCode_paStreamIsNotStopped,
  PaInputOverflowed = raw_portaudio::PaErrorCode_paInputOverflowed,
  PaOutputUnderflowed = raw_portaudio::PaErrorCode_paOutputUnderflowed,
  PaHostApiNotFound = raw_portaudio::PaErrorCode_paHostApiNotFound,
  PaInvalidHostApi = raw_portaudio::PaErrorCode_paInvalidHostApi,
  PaCanNotReadFromACallbackStream = raw_portaudio::PaErrorCode_paCanNotReadFromACallbackStream,
  PaCanNotWriteToACallbackStream = raw_portaudio::PaErrorCode_paCanNotWriteToACallbackStream,
  PaCanNotReadFromAnOutputOnlyStream = raw_portaudio::PaErrorCode_paCanNotReadFromAnOutputOnlyStream,
  PaCanNotWriteToAnInputOnlyStream = raw_portaudio::PaErrorCode_paCanNotWriteToAnInputOnlyStream,
  PaIncompatibleStreamHostApi = raw_portaudio::PaErrorCode_paIncompatibleStreamHostApi,
  PaBadBufferPtr = raw_portaudio::PaErrorCode_paBadBufferPtr,
  UnknownError,
}


impl PaError {
  /// Get the enum value corresponding to the given i32
  pub fn from_i32(num: i32) -> PaError {
    match num {
      raw_portaudio::PaErrorCode_paNoError => PaError::PaNoError,
      raw_portaudio::PaErrorCode_paNotInitialized => PaError::PaNotInitialized,
      raw_portaudio::PaErrorCode_paUnanticipatedHostError => PaError::PaUnanticipatedHostError,
      raw_portaudio::PaErrorCode_paInvalidChannelCount => PaError::PaInvalidChannelCount,
      raw_portaudio::PaErrorCode_paInvalidSampleRate => PaError::PaInvalidSampleRate,
      raw_portaudio::PaErrorCode_paInvalidDevice => PaError::PaInvalidDevice,
      raw_portaudio::PaErrorCode_paInvalidFlag => PaError::PaInvalidFlag,
      raw_portaudio::PaErrorCode_paSampleFormatNotSupported => PaError::PaSampleFormatNotSupported,
      raw_portaudio::PaErrorCode_paBadIODeviceCombination => PaError::PaBadIODeviceCombination,
      raw_portaudio::PaErrorCode_paInsufficientMemory => PaError::PaInsufficientMemory,
      raw_portaudio::PaErrorCode_paBufferTooBig => PaError::PaBufferTooBig,
      raw_portaudio::PaErrorCode_paBufferTooSmall => PaError::PaBufferTooSmall,
      raw_portaudio::PaErrorCode_paNullCallback => PaError::PaNullCallback,
      raw_portaudio::PaErrorCode_paBadStreamPtr => PaError::PaBadStreamPtr,
      raw_portaudio::PaErrorCode_paTimedOut => PaError::PaTimedOut,
      raw_portaudio::PaErrorCode_paInternalError => PaError::PaInternalError,
      raw_portaudio::PaErrorCode_paDeviceUnavailable => PaError::PaDeviceUnavailable,
      raw_portaudio::PaErrorCode_paIncompatibleHostApiSpecificStreamInfo => PaError::PaIncompatibleHostApiSpecificStreamInfo,
      raw_portaudio::PaErrorCode_paStreamIsStopped => PaError::PaStreamIsStopped,
      raw_portaudio::PaErrorCode_paStreamIsNotStopped => PaError::PaStreamIsNotStopped,
      raw_portaudio::PaErrorCode_paInputOverflowed => PaError::PaInputOverflowed,
      raw_portaudio::PaErrorCode_paOutputUnderflowed => PaError::PaOutputUnderflowed,
      raw_portaudio::PaErrorCode_paHostApiNotFound => PaError::PaHostApiNotFound,
      raw_portaudio::PaErrorCode_paInvalidHostApi => PaError::PaInvalidHostApi,
      raw_portaudio::PaErrorCode_paCanNotReadFromACallbackStream => PaError::PaCanNotReadFromACallbackStream,
      raw_portaudio::PaErrorCode_paCanNotWriteToACallbackStream => PaError::PaCanNotWriteToACallbackStream,
      raw_portaudio::PaErrorCode_paCanNotReadFromAnOutputOnlyStream => PaError::PaCanNotReadFromAnOutputOnlyStream,
      raw_portaudio::PaErrorCode_paCanNotWriteToAnInputOnlyStream => PaError::PaCanNotWriteToAnInputOnlyStream,
      raw_portaudio::PaErrorCode_paIncompatibleStreamHostApi => PaError::PaIncompatibleStreamHostApi,
      raw_portaudio::PaErrorCode_paBadBufferPtr => PaError::PaBadBufferPtr,
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

