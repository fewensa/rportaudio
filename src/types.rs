use std::ffi::CStr;
use std::time::Duration;

use libc::{c_ulong, c_void};

use crate::{kit, raw_portaudio};
use crate::rpa_error::PaError;

/// Index number of a Host API
pub type HostApiIndex = u32;


/// Index of a Device
pub type DeviceIndex = u32;


//#[derive(Debug, Clone)]
//pub struct PaVersionInfo {
////  pub(crate) pa_version_info: *const raw_portaudio::PaVersionInfo
//
//  pub major: i32,
//  pub minor: i32,
//  pub sub_minor: i32,
//  #[doc = "This is currently the Git revision hash but may change in the future."]
//  #[doc = "The versionControlRevision is updated by running a script before compiling the library."]
//  #[doc = "If the update does not occur, this value may refer to an earlier revision."]
//  pub control_revision: String,
//  #[doc = " Version as a string, for example \"PortAudio V19.5.0-devel, revision 1952M\""]
//  pub text: String,
//}
//
//impl PaVersionInfo {
//  pub(crate) fn from_raw(raw: &raw_portaudio::PaVersionInfo) -> PaVersionInfo {
//    PaVersionInfo {
//      major: raw.versionMajor,
//      minor: raw.versionMinor,
//      sub_minor: raw.versionSubMinor,
//      control_revision: String::from_utf8_lossy(unsafe { CStr::from_ptr(raw.versionControlRevision).to_bytes() }).into_owned(),
//      text: String::from_utf8_lossy(unsafe { CStr::from_ptr(raw.versionText).to_bytes() }).into_owned(),
//    }
//  }
//}


/// Possible Host API types
#[repr(u32)]
#[derive(Debug, Copy, Clone)]
#[allow(missing_docs)]
pub enum HostApiType {
  InDevelopment = raw_portaudio::paInDevelopment,
  DirectSound = raw_portaudio::paDirectSound,
  MME = raw_portaudio::paMME,
  ASIO = raw_portaudio::paASIO,
  SoundManager = raw_portaudio::paSoundManager,
  CoreAudio = raw_portaudio::paCoreAudio,
  OSS = raw_portaudio::paOSS,
  ALSA = raw_portaudio::paALSA,
  AL = raw_portaudio::paAL,
  BeOS = raw_portaudio::paBeOS,
  WDMKS = raw_portaudio::paWDMKS,
  JACK = raw_portaudio::paJACK,
  WASAPI = raw_portaudio::paWASAPI,
  AudioScienceHPI = raw_portaudio::paAudioScienceHPI,

  /// Added for when FromPrimitive returns None
  Unknown,
}


impl HostApiType {
  /// Convert a static host API unique identifier, into a runtime host API index.
  pub fn to_api_index(self) -> Result<u32, PaError> {
    crate::rportaudio::hostapi_type_id_to_hostapi_index(self as u32)
  }

  /// Get the enum value corresponding to the u32
  pub fn from_u32(num: u32) -> HostApiType {
    match num {
      raw_portaudio::paInDevelopment => HostApiType::InDevelopment,
      raw_portaudio::paDirectSound => HostApiType::DirectSound,
      raw_portaudio::paMME => HostApiType::MME,
      raw_portaudio::paASIO => HostApiType::ASIO,
      raw_portaudio::paSoundManager => HostApiType::SoundManager,
      raw_portaudio::paCoreAudio => HostApiType::CoreAudio,
      raw_portaudio::paOSS => HostApiType::OSS,
      raw_portaudio::paALSA => HostApiType::ALSA,
      raw_portaudio::paAL => HostApiType::AL,
      raw_portaudio::paBeOS => HostApiType::BeOS,
      raw_portaudio::paWDMKS => HostApiType::WDMKS,
      raw_portaudio::paJACK => HostApiType::JACK,
      raw_portaudio::paWASAPI => HostApiType::WASAPI,
      raw_portaudio::paAudioScienceHPI => HostApiType::AudioScienceHPI,
      _ => HostApiType::Unknown,
    }
  }
}


#[derive(Debug, Clone)]
pub struct PaHostApiInfo {
  #[doc = " this is struct version 1"]
  pub struct_version: i32,
  #[doc = " The well known unique identifier of this host API @see PaHostApiTypeId"]
  pub type_: HostApiType,
  #[doc = " A textual description of the host API for display on user interfaces."]
  pub name: String,
  #[doc = "  The number of devices belonging to this host API. This field may be"]
  #[doc = "used in conjunction with Pa_HostApiDeviceIndexToDeviceIndex() to enumerate"]
  #[doc = "all devices for this host API."]
  #[doc = "@see Pa_HostApiDeviceIndexToDeviceIndex"]
  pub device_count: u32,
  #[doc = " The default input device for this host API. The value will be a"]
  #[doc = "device index ranging from 0 to (Pa_GetDeviceCount()-1), or paNoDevice"]
  #[doc = "if no default input device is available."]
  pub default_input: Option<u32>,
  #[doc = " The default output device for this host API. The value will be a"]
  #[doc = "device index ranging from 0 to (Pa_GetDeviceCount()-1), or paNoDevice"]
  #[doc = "if no default output device is available."]
  pub default_output: Option<u32>,
}

impl PaHostApiInfo {
  pub(crate) fn from_raw(raw: &raw_portaudio::PaHostApiInfo) -> PaHostApiInfo {
    Self {
      struct_version: raw.structVersion,
      type_: HostApiType::from_u32(raw._type),
      name: String::from_utf8_lossy(unsafe { CStr::from_ptr(raw.name).to_bytes() }).into_owned(),
      device_count: raw.deviceCount as u32,
      default_input: match raw.defaultInputDevice {
        n if n >= 0 => Some(n as u32),
        _ => None
      },
      default_output: match raw.defaultOutputDevice {
        n if n >= 0 => Some(n as u32),
        _ => None
      },
    }
  }
}


/// Error info obtained by get_last_error
#[derive(Debug, Clone)]
pub struct PaHostErrorInfo {
  /// The error code given
  pub code: i32,
  /// A human readable error message
  pub text: String,
  /// The type of the API that produced the error
  pub api_type: HostApiType,
}

impl PaHostErrorInfo {
  pub(crate) fn from_raw(raw: &raw_portaudio::PaHostErrorInfo) -> Self {
    Self {
      code: raw.errorCode as i32,
      text: String::from_utf8_lossy(unsafe { CStr::from_ptr(raw.errorText).to_bytes() }).into_owned(),
      api_type: HostApiType::from_u32(raw.hostApiType),
    }
  }
}


/// Information for a specific device
pub struct PaDeviceInfo {
  pub struct_version: i32,

  /// Human readable name
  pub name: String,

  /// Index of the host API this device belongs to
  pub host_api: u32,

  /// Maximal number of input channels that can be used
  pub max_input_channels: u32,

  /// Maximal number of ouput channels that can be used
  pub max_output_channels: u32,

  /// Default input latency for interactive performance
  pub default_low_input_latency: Duration,

  /// Default output latency for interactive performance
  pub default_low_output_latency: Duration,

  /// Default input latency for robust non-interactive applications
  pub default_high_input_latency: Duration,

  /// Default output latency for robust non-interactive applications
  pub default_high_output_latency: Duration,

  /// Default sample rate
  pub default_sample_rate: f64,
}

impl PaDeviceInfo {
  pub(crate) fn from_raw(raw: &raw_portaudio::PaDeviceInfo) -> Self {
    Self {
      struct_version: raw.structVersion,
      name: String::from_utf8_lossy(unsafe { CStr::from_ptr(raw.name).to_bytes() }).into_owned(),
      host_api: raw.hostApi as u32,
      max_input_channels: raw.maxInputChannels as u32,
      max_output_channels: raw.maxOutputChannels as u32,
      default_low_input_latency: kit::pa_time_to_duration(raw.defaultLowInputLatency),
      default_low_output_latency: kit::pa_time_to_duration(raw.defaultLowOutputLatency),
      default_high_input_latency: kit::pa_time_to_duration(raw.defaultHighInputLatency),
      default_high_output_latency: kit::pa_time_to_duration(raw.defaultHighOutputLatency),
      default_sample_rate: raw.defaultSampleRate,
    }
  }
}


/// Types that are allowed to be used as samples in a Stream
///
/// *WARNING*: It is not advised to implement this trait for any other types as the size and flag
/// may not be the correct one.
pub trait SampleType {
  /// Should return the PortAudio flag which corresponds to the type
  fn sample_format() -> u64;
}

impl SampleType for f32 { fn sample_format() -> u64 { 0x00000001 } }

impl SampleType for i32 { fn sample_format() -> u64 { 0x00000002 } }

impl SampleType for i16 { fn sample_format() -> u64 { 0x00000008 } }

impl SampleType for i8 { fn sample_format() -> u64 { 0x00000010 } }

impl SampleType for u8 { fn sample_format() -> u64 { 0x00000020 } }


/// Stream parameters to be used with Stream::open()
#[derive(Copy, Clone)]
pub struct PaStreamParameters<T> {
  /// Index of the device to use
  pub device: DeviceIndex,

  /// Requested number of channels
  pub channel_count: u32,

  /// Desired latency of the stream
  pub suggested_latency: Duration,

  /// Sample data to be used in the stream
  pub data: T,
}

impl<T: SampleType> PaStreamParameters<T> {
  pub(crate) fn to_raw(&self) -> raw_portaudio::PaStreamParameters {
    raw_portaudio::PaStreamParameters {
      device: self.device as i32,
      channelCount: self.channel_count as i32,
      sampleFormat: <T as SampleType>::sample_format() as raw_portaudio::PaSampleFormat,
      suggestedLatency: kit::duration_to_pa_time(self.suggested_latency),
      hostApiSpecificStreamInfo: ::std::ptr::null_mut(),
    }
  }
}


/// Time information for various stream related values
#[derive(Copy, Clone)]
pub struct PaStreamTimeInfo {
  /// Timestamp for the ADC capture time of the first frame
  pub input_adc_time: Duration,

  /// Timestamp that the callback was invoked
  pub current_time: Duration,

  /// Timestamp for the DAC output time of the first frame
  pub output_dac_time: Duration,
}

impl PaStreamTimeInfo {
  pub(crate) fn from_raw(data: &raw_portaudio::PaStreamCallbackTimeInfo) -> Self {
    Self {
      input_adc_time: kit::pa_time_to_duration(data.inputBufferAdcTime),
      current_time: kit::pa_time_to_duration(data.currentTime),
      output_dac_time: kit::pa_time_to_duration(data.outputBufferDacTime),
    }
  }
}


/// Information about the actual latency and sample rate values the stream uses
#[derive(Copy, Clone)]
pub struct PaStreamInfo {
  /// Input latency
  pub input_latency: Duration,

  /// Output latency
  pub output_latency: Duration,

  /// Sample rate
  pub sample_rate: f64,
}

impl PaStreamInfo {
  pub(crate) fn from_raw(data: &raw_portaudio::PaStreamInfo) -> Self {
    Self {
      input_latency: kit::pa_time_to_duration(data.inputLatency),
      output_latency: kit::pa_time_to_duration(data.outputLatency),
      sample_rate: data.sampleRate,
    }
  }
}


/// Callback to consume, process or generate audio
pub type StreamCallback<'a, I, O> = FnMut(&[I], &mut [O], PaStreamTimeInfo, PaStreamCallbackFlags) -> PaStreamCallbackResult + 'a;




bitflags!(
  #[doc="Flags indicating the status of the callback"]
  flags PaStreamCallbackFlags: u64 {
    #[doc="Indicates that the callback has inserted one or more zeroes since not enough data was available"]
    const INPUT_UNDERFLOW = 0x01,

    #[doc="Indicates that the callback has discarded some data"]
    const INPUT_OVERFLOW = 0x02,

    #[doc="Indicates that extra data was inserted in the output since there was not engough available"]
    const OUTPUT_UNDERFLOW = 0x04,

    #[doc="Indicates that certain data was discarded since there was no room"]
    const OUTPUT_OVERFLOW = 0x08,

    #[doc="Some or all of the output data will be used to prime the stream, input data may be zero"]
    const PRIMING_OUTPUT = 0x10
  }
);

bitflags!(
  #[doc="Flags used to control the behavior of a stream"]
  flags StreamFlags: u64 {
    #[doc="Disable clipping of out of range samples"]
    const CLIP_OFF                                   = 0x00000001,

    #[doc="Disable dithering"]
    const DITHER_OFF                                 = 0x00000002,

    #[doc="Request that a full duplex stream will not discard overflowed input samples. The frames_per_buffer must be set to unspecified (0)"]
    const NEVER_DROP_INPUT                           = 0x00000004,

    #[doc="Call the stream callback to fill initial output buffers, rather than priming the buffers with silence"]
    const PRIME_OUTPUT_BUFFERS_USING_STREAM_CALLBACK = 0x00000008,

    #[doc="Range for platform specific flags. Not all of the upper 16 bits need to be set at the same time."]
    const PLATFORM_SPECIFIC                          = 0xFFFF0000
  }
);



#[repr(u32)]
#[derive(Copy, Clone)]
pub enum PaStreamCallbackResult {
  /// Continue invoking the callback
  Continue = raw_portaudio::paContinue,

  /// Stop invoking the callback and finish once everything has played
  Complete = raw_portaudio::paComplete,

  /// Stop invoking the callback and finish as soon as possible
  Abort = raw_portaudio::paAbort,
}


/// Callback to be fired when a StreamCallback is stopped
pub type StreamFinishedCallback<'a> = FnMut() + 'a;

pub(crate) struct StreamUserData<'a, I, O> {
  pub(crate) num_input: u32,
  pub(crate) num_output: u32,
  pub(crate) callback: Option<Box<StreamCallback<'a, I, O>>>,
  pub(crate) finished_callback: Option<Box<StreamFinishedCallback<'a>>>,
}


/// An object for an PortAudio stream
///
/// Streams can have an input type I and output type O.
pub struct Stream<'a, I: SampleType, O: SampleType> {
  pub(crate) pa_stream: *mut raw_portaudio::PaStream,
  pub(crate) inputs: u32,
  pub(crate) outputs: u32,
  pub(crate) user_data: Box<StreamUserData<'a, I, O>>,
}


