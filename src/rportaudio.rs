use std::{mem, ptr};
use std::ffi::CStr;
use std::time::Duration;

use libc::{c_ulong, c_void};

use crate::{kit, raw_portaudio};
use crate::rpa_error::{PaError, PaResult};
use crate::types::*;

/// PortAudio version
pub fn version() -> i32 {
  let version = unsafe { raw_portaudio::Pa_GetVersion() };
  version as i32
}

/// Human-readable PortAudio version
pub fn version_text() -> String {
  let version_c = unsafe { raw_portaudio::Pa_GetVersionText() };
  let version_s = String::from_utf8_lossy(unsafe { CStr::from_ptr(version_c).to_bytes() });
  version_s.into_owned()
}

/// Initialize the PortAudio API
///
/// Each successful call must be matched by a call to terminate
pub fn initialize() -> PaResult {
  kit::to_pa_result(unsafe { raw_portaudio::Pa_Initialize() })
}

/// Terminate the PortAudio API
///
/// Call this function exactly once for each successful call to initialize
pub fn terminate() -> PaResult {
  kit::to_pa_result(unsafe { raw_portaudio::Pa_Terminate() })
}

pub fn version_info() -> PaVersionInfo {
  unsafe {
    let pa_version_info = raw_portaudio::Pa_GetVersionInfo();
    PaVersionInfo::from_raw(&*pa_version_info)
  }
}

pub fn error_text(pa_error: PaError) -> String {
  match pa_error {
    PaError::UnknownError => "Unknown Error".to_string(),
    other => {
      let message_c = unsafe { raw_portaudio::Pa_GetErrorText(other as i32) };
      let message_s = String::from_utf8_lossy(unsafe { CStr::from_ptr(message_c).to_bytes() });
      message_s.into_owned()
    }
  }
}


/// Get the number of host API's available
pub fn hostapi_count() -> Result<u32, PaError> {
  match unsafe { raw_portaudio::Pa_GetHostApiCount() } {
    n if n >= 0 => Ok(n as HostApiIndex),
    m => kit::to_pa_result(m).map(|_| 0),
  }
}

/// Get the default Host API
pub fn default_hostapi() -> Result<HostApiIndex, PaError> {
  match unsafe { raw_portaudio::Pa_GetDefaultHostApi() } {
    n if n >= 0 => Ok(n as HostApiIndex),
    m => kit::to_pa_result(m).map(|_| 0),
  }
}


/// Get information about a specific Host API
///
/// Returns None when an invalid index is given
pub fn hostapi_info(index: HostApiIndex) -> Option<PaHostApiInfo> {
  unsafe {
    match raw_portaudio::Pa_GetHostApiInfo(index as i32) {
      p if p.is_null() => None,
      p => Some(PaHostApiInfo::from_raw(&*p)),
    }
  }
}

pub fn hostapi_type_id_to_hostapi_index(type_id: u32) -> Result<u32, PaError> {
  match unsafe { raw_portaudio::Pa_HostApiTypeIdToHostApiIndex(type_id) } {
    n if n >= 0 => Ok(n as u32),
    m => kit::to_pa_result(m).map(|_| 0),
  }
}


/// Return information about the last host error encountered.
///
/// The values in this structure will only be valid if a PortAudio function has previously returned
/// the UnanticipatedHostError error code.
pub fn last_host_error() -> Option<PaHostErrorInfo> {
  unsafe {
    match raw_portaudio::Pa_GetLastHostErrorInfo() {
      p if p.is_null() => None,
      p => Some(PaHostErrorInfo::from_raw(&*p)),
    }
  }
}


/// Retrieve the number of available devices.
pub fn device_count() -> Result<u32, PaError> {
  match unsafe { raw_portaudio::Pa_GetDeviceCount() } {
    n if n >= 0 => Ok(n as u32),
    m => kit::to_pa_result(m).map(|_| 0),
  }
}


/// Retrieve the index of the default input device
///
/// Will return None when none are available.
pub fn default_input_device() -> Option<DeviceIndex> {
  match unsafe { raw_portaudio::Pa_GetDefaultInputDevice() } {
    n if n >= 0 => Some(n as u32),
    _ => None,
  }
}


/// Retrieve the index of the default output device
///
/// Will return None when none are available.
pub fn default_output_device() -> Option<DeviceIndex> {
  match unsafe { raw_portaudio::Pa_GetDefaultOutputDevice() } {
    n if n >= 0 => Some(n as u32),
    _ => None,
  }
}


/// Get info about a particular device
///
/// Returns None when the index is out of range.
pub fn device_info(index: DeviceIndex) -> Option<PaDeviceInfo> {
  unsafe {
    match raw_portaudio::Pa_GetDeviceInfo(index as i32) {
      p if p.is_null() => None,
      p => Some(PaDeviceInfo::from_raw(&*p)),
    }
  }
}


/// Converts a device index from a specific host API to a global device index
///
/// Returns Err(InvalidHostApi) when the host_api is out of range, and Err(InvalidDevice) when
/// host_api_device_index is out of range.
///
/// ```
/// // We retrieve the index of device 3 of api 1
/// let device_index = match rportaudio::hostapi_device_index_to_device_index(1, 3) {
///   Ok(n) => n,
///   Err(e) => { println!("Error: {:?}", e); return; }
/// };
/// ```
pub fn hostapi_device_index_to_device_index(hostapi: u32, hostapi_device_index: u32) -> Result<u32, PaError> {
  match unsafe { raw_portaudio::Pa_HostApiDeviceIndexToDeviceIndex(hostapi as i32, hostapi_device_index as i32) } {
    n if n >= 0 => Ok(n as u32),
    m => kit::to_pa_result(m).map(|_| 0),
  }
}


/// Returns Ok when the StreamParameters are supported. This ignores the latency field.
pub fn is_format_supported<I: SampleType, O: SampleType>(input: Option<PaStreamParameters<I>>, output: Option<PaStreamParameters<O>>, sample_rate: f64) -> PaResult {
  let input_obj;
  let input_ptr;
  let output_obj;
  let output_ptr;
  match input {
    Some(sp) => {
      input_obj = sp.to_raw();
      input_ptr = &input_obj as *const _
    }
    None => input_ptr = ptr::null(),
  };
  match output {
    Some(sp) => {
      output_obj = sp.to_raw();
      output_ptr = &output_obj as *const _
    }
    None => output_ptr = ptr::null(),
  };

  kit::to_pa_result(unsafe { raw_portaudio::Pa_IsFormatSupported(input_ptr, output_ptr, sample_rate) })
}


pub fn open_stream<'a, I, O>(
  input: Option<PaStreamParameters<I>>,
  output: Option<PaStreamParameters<O>>,
  sample_rate: f64,
  frames_per_buffer: u64,
  flags: StreamFlags,
  callback: Option<Box<StreamCallback<'a, I, O>>>,
) -> Result<Stream<'a, I, O>, PaError>
  where I: SampleType, O: SampleType {
  let callback_pointer = match callback {
    Some(_) => Some(stream_callback::<I, O> as StreamCallbackType),
    None => None,
  };

  let input_cnt;
  let input_obj;
  let input_ptr;
  let output_cnt;
  let output_obj;
  let output_ptr;
  match input {
    Some(sp) => {
      input_cnt = sp.channel_count;
      input_obj = sp.to_raw();
      input_ptr = &input_obj as *const _
    }
    None => {
      input_cnt = 0;
      input_ptr = ptr::null()
    }
  };
  match output {
    Some(sp) => {
      output_cnt = sp.channel_count;
      output_obj = sp.to_raw();
      output_ptr = &output_obj as *const _
    }
    None => {
      output_cnt = 0;
      output_ptr = ptr::null()
    }
  };

  let mut user_data = Box::new(StreamUserData {
    num_input: input_cnt,
    num_output: output_cnt,
    callback,
    finished_callback: None,
  });

  let mut pa_stream = ::std::ptr::null_mut();
  let pointer_for_callback: *mut c_void = &mut *user_data as *mut StreamUserData<I, O> as *mut c_void;

  let result = unsafe {
    raw_portaudio::Pa_OpenStream(&mut pa_stream,
                                 input_ptr,
                                 output_ptr,
                                 sample_rate,
                                 frames_per_buffer as c_ulong,
                                 flags.bits() as c_ulong,
                                 callback_pointer,
                                 pointer_for_callback)
  };
  match kit::to_pa_result(result) {
    Ok(()) => Ok(Stream {
      pa_stream,
      user_data,
      inputs: input_cnt,
      outputs: output_cnt,
    }),
    Err(v) => Err(v),
  }
}


pub fn open_default_stream<'a, T>(
  num_input_channels: u32,
  num_output_channels: u32,
  sample_rate: f64,
  frames_per_buffer: u64,
  callback: Option<Box<StreamCallback<'a, T, T>>>,
) -> Result<Stream<'a, T, T>, PaError>
  where T: SampleType {
  let callback_pointer = match callback {
    Some(_) => Some(stream_callback::<T, T> as StreamCallbackType),
    None => None,
  };
  let mut userdata = Box::new(StreamUserData {
    num_input: num_input_channels,
    num_output: num_output_channels,
    callback,
    finished_callback: None,
  });
  let mut pa_stream = ::std::ptr::null_mut();

  let pointer_for_callback: *mut c_void = &mut *userdata as *mut StreamUserData<T, T> as *mut c_void;

  let code = unsafe {
    raw_portaudio::Pa_OpenDefaultStream(&mut pa_stream,
                                        num_input_channels as i32,
                                        num_output_channels as i32,
                                        <T as SampleType>::sample_format() as c_ulong,
                                        sample_rate,
                                        frames_per_buffer as c_ulong,
                                        callback_pointer,
                                        pointer_for_callback)
  };

  match kit::to_pa_result(code) {
    Ok(()) => Ok(Stream {
      pa_stream,
      user_data: userdata,
      inputs: num_input_channels,
      outputs: num_output_channels,
    }),
    Err(v) => Err(v),
  }
}


/// Set a callback which is to be called when the StreamCallback finishes
pub fn set_stream_finished_callback<'a, I, O>(stream: &mut Stream<I, O>, finished_callback: Box<StreamFinishedCallback<'a>>) -> PaResult
  where
    I: SampleType, O: SampleType {
  stream.user_data.finished_callback = Some(finished_callback);
  let callback_pointer = Some(stream_finished_callback::<I, O> as StreamFinishedCallbackType);
  kit::to_pa_result(unsafe { raw_portaudio::Pa_SetStreamFinishedCallback(stream.pa_stream, callback_pointer) })
}

/// Remove any previously attached finish callback
pub fn unset_stream_finished_callback<I, O>(stream: &mut Stream<I, O>) -> PaResult
  where
    I: SampleType, O: SampleType {
  stream.user_data.finished_callback = None;
  kit::to_pa_result(unsafe { raw_portaudio::Pa_SetStreamFinishedCallback(stream.pa_stream, None) })
}

/// Starts the stream
pub fn start_stream<I, O>(stream: &Stream<I, O>) -> PaResult
  where
    I: SampleType, O: SampleType {
  kit::to_pa_result(unsafe { raw_portaudio::Pa_StartStream(stream.pa_stream) })
}


/// Stops the stream. It will block untill all audio has finished playing
pub fn stop_stream<I, O>(stream: &Stream<I, O>) -> PaResult
  where
    I: SampleType, O: SampleType {
  kit::to_pa_result(unsafe { raw_portaudio::Pa_StopStream(stream.pa_stream) })
}

/// Stop stream immediately without waiting for the buffers to complete
pub fn abort_stream<I, O>(stream: &Stream<I, O>) -> PaResult
  where
    I: SampleType, O: SampleType {
  kit::to_pa_result(unsafe { raw_portaudio::Pa_AbortStream(stream.pa_stream) })
}


pub fn close_stream<I, O>(stream: &Stream<I, O>) -> PaResult
  where
    I: SampleType, O: SampleType {
  kit::to_pa_result(unsafe { raw_portaudio::Pa_CloseStream(stream.pa_stream) })
}


/// Returns wether the stream is stopped
pub fn is_stream_stopped<I, O>(stream: &Stream<I, O>) -> Result<bool, PaError>
  where
    I: SampleType, O: SampleType {
  match unsafe { raw_portaudio::Pa_IsStreamStopped(stream.pa_stream) } {
    1 => Ok(true),
    n => kit::to_pa_result(n).map(|_| false),
  }
}

/// Returns wether the stream is active
pub fn is_stream_active<I, O>(stream: &Stream<I, O>) -> Result<bool, PaError>
  where
    I: SampleType, O: SampleType {
  match unsafe { raw_portaudio::Pa_IsStreamActive(stream.pa_stream) } {
    1 => Ok(true),
    n => kit::to_pa_result(n).map(|_| false),
  }
}


/// Get the number of frames that can be read from the stream without waiting
pub fn stream_num_read_available<I, O>(stream: &Stream<I, O>) -> Result<u32, PaError>
  where
    I: SampleType, O: SampleType {
  match unsafe { raw_portaudio::Pa_GetStreamReadAvailable(stream.pa_stream) } {
    n if n >= 0 => { Ok(n as u32) }
    n => kit::to_pa_result(n as i32).map(|_| 0),
  }
}

/// Get the number of frames that can be written to the stream without waiting
pub fn stream_num_write_available<I, O>(stream: &Stream<I, O>) -> Result<u32, PaError>
  where
    I: SampleType, O: SampleType {
  match unsafe { raw_portaudio::Pa_GetStreamWriteAvailable(stream.pa_stream) } {
    n if n >= 0 => { Ok(n as u32) }
    n => kit::to_pa_result(n as i32).map(|_| 0),
  }
}


/// Write the given buffer to the stream. This function blocks
///
/// Possible Error codes:
///
/// * `CanNotWriteToAnInputOnlyStream`: when num_output_channels = 0
/// * `BadBufferPtr`: when buffer.len() is not a multiple of num_output_channels
/// * Some other error given by PortAudio
pub fn write_stream<I, O>(stream: &Stream<I, O>, buffer: &[O]) -> PaResult
  where
    I: SampleType, O: SampleType {
  if stream.outputs == 0 {
    return Err(PaError::PaCanNotWriteToAnInputOnlyStream);
  }

  // Ensure the buffer is the correct size.
  if buffer.len() % stream.outputs as usize != 0 {
    return Err(PaError::PaBadBufferPtr);
  }

  let pointer = buffer.as_ptr() as *const c_void;
  let frames = (buffer.len() / stream.outputs as usize) as c_ulong;

  kit::to_pa_result(unsafe { raw_portaudio::Pa_WriteStream(stream.pa_stream, pointer, frames) })
}

/// Reads the requested number of frames from the input devices. This function blocks until
/// the whole buffer has been filled.
///
/// Will return `CanNotReadFromAnOutputOnlyStream` if num_input_channels = 0.
pub fn read_stream<I, O>(stream: &Stream<I, O>, frames: u32) -> Result<Vec<I>, PaError>
  where
    I: SampleType, O: SampleType {
  if stream.inputs == 0 { return Err(PaError::PaCanNotReadFromAnOutputOnlyStream); }

  // We create a buffer with the needed capacity. Then we feed that to the library, which
  // will fill the buffer accordingly. Afterwards, we set the length of the vector as all its
  // elements are now initialized.
  let vec_len = frames * stream.inputs;
  let mut buffer = Vec::with_capacity(vec_len as usize);

  let buffer_ptr = buffer.as_mut_ptr() as *mut c_void;
  match kit::to_pa_result(unsafe { raw_portaudio::Pa_ReadStream(stream.pa_stream, buffer_ptr, frames as c_ulong) }) {
    Ok(()) => {
      unsafe { buffer.set_len(vec_len as usize); }
      Ok(buffer)
    }
    Err(e) => Err(e),
  }
}

/// Returns the cpu load the stream callback consumes. This will return 0.0 if the stream uses
/// blocking read/write, or if an error occured.
pub fn stream_cpu_load<I, O>(stream: &Stream<I, O>) -> f64
  where
    I: SampleType, O: SampleType {
  unsafe { raw_portaudio::Pa_GetStreamCpuLoad(stream.pa_stream) }
}

/// Get the current timestamp of the stream
pub fn stream_time<I, O>(stream: &Stream<I, O>) -> Duration
  where
    I: SampleType, O: SampleType {
  let time = unsafe { raw_portaudio::Pa_GetStreamTime(stream.pa_stream) };
  kit::pa_time_to_duration(time)
}

/// Get the actual latencies and sample rate
///
/// Returns None when the stream is invalid or an error occured
pub fn stream_info<I, O>(stream: &Stream<I, O>) -> Option<PaStreamInfo>
  where
    I: SampleType, O: SampleType {
  unsafe {
    match raw_portaudio::Pa_GetStreamInfo(stream.pa_stream) {
      p if p.is_null() => None,
      p => Some(PaStreamInfo::from_raw(&*p)),
    }
  }
}

pub fn sample_size<T: SampleType>() -> Result<u32, PaError> {
  match unsafe { raw_portaudio::Pa_GetSampleSize(<T as SampleType>::sample_format() as c_ulong) } {
    n if n >= 0 => Ok(n as u32),
    m => kit::to_pa_result(m).map(|_| 0),
  }
}


pub fn sleep(msec: i32) {
  unsafe {
    raw_portaudio::Pa_Sleep(msec as ::std::os::raw::c_long);
  }
}


type StreamCallbackType = extern "C" fn(*const c_void, *mut c_void, ::libc::c_ulong, *const raw_portaudio::PaStreamCallbackTimeInfo, raw_portaudio::PaStreamCallbackFlags, *mut c_void) -> ::libc::c_int;
type StreamFinishedCallbackType = extern "C" fn(*mut c_void);

extern "C" fn stream_callback<I, O>(input: *const c_void,
                                    output: *mut c_void,
                                    frame_count: ::libc::c_ulong,
                                    time_info: *const raw_portaudio::PaStreamCallbackTimeInfo,
                                    status_flags: raw_portaudio::PaStreamCallbackFlags,
                                    user_data: *mut c_void) -> ::libc::c_int {
  // TODO: use Box::from_raw once it is stable
  let mut stream_data: Box<StreamUserData<I, O>> = unsafe { mem::transmute(user_data) };
  let input_buffer: &[I] = unsafe {
    ::std::slice::from_raw_parts(input as *const I, frame_count as usize * stream_data.num_input as usize)
  };
  let output_buffer: &mut [O] = unsafe {
    ::std::slice::from_raw_parts_mut(output as *mut O, frame_count as usize * stream_data.num_output as usize)
  };

  let flags = PaStreamCallbackFlags::from_bits_truncate(status_flags as u64);

  assert!(!time_info.is_null());
  let time_info_ll = unsafe { &*time_info };
  let timeinfo = PaStreamTimeInfo::from_raw(time_info_ll);

  let result = match stream_data.callback {
    Some(ref mut f) => (*f)(input_buffer, output_buffer, timeinfo, flags),
    None => PaStreamCallbackResult::Abort,
  };

  mem::forget(stream_data);

  result as i32
}

extern "C" fn stream_finished_callback<I, O>(user_data: *mut c_void) {
  // TODO: use Box::from_raw once it is stable
  let mut stream_data: Box<StreamUserData<I, O>> = unsafe { mem::transmute(user_data) };
  match stream_data.finished_callback {
    Some(ref mut f) => (*f)(),
    None => {}
  };

  mem::forget(stream_data);
}

