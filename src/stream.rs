use std::time::Duration;

pub use crate::types::Stream;
use crate::rpa_error::{PaError, PaResult};
use crate::rportaudio;
use crate::types::*;

/// Argument to Stream::open() or Stream::open_default() to allow PortAudio itself determine the
/// optimal number of frames per buffer. This number may differ each time the callback is called.
pub const FRAMES_PER_BUFFER_UNSPECIFIED: u64 = 0;

impl<'a, T: SampleType> Stream<'a, T, T> {
  /// Constructs a stream using the default input and output devices
  ///
  /// ## Arguments
  /// * num_input_channels: Desired number of input channels
  /// * num_output_channels: Desired number of output channels
  /// * sample_rate: Sample rate of the stream
  /// * frames_per_buffer: Number of frames per buffer. Use FRAMES_PER_BUFFER_UNSPECIFIED to let
  /// portaudio determine the optimal number.
  /// * callback: Some(callback) which PortAudio will call to read/write the buffers, or None
  /// when using the read and write methods
  pub fn open_default(num_input_channels: u32,
                      num_output_channels: u32,
                      sample_rate: f64,
                      frames_per_buffer: u64,
                      callback: Option<Box<StreamCallback<'a, T, T>>>)
                      -> Result<Stream<'a, T, T>, PaError> {
    rportaudio::open_default_stream(
      num_input_channels,
      num_output_channels,
      sample_rate,
      frames_per_buffer,
      callback,
    )
  }
}


impl<'a, I: SampleType, O: SampleType> Stream<'a, I, O> {
  /// Constructs a stream with the desired input and output specifications
  ///
  /// ## Arguments
  /// * input: Specification for the input channel, or None for an output-only stream
  /// * output: Specification for the output channel, or None for an input-only stream
  /// * sample_rate: Sample rate of the stream
  /// * frames_per_buffer: Number of frames per buffer. Use FRAMES_PER_BUFFER_UNSPECIFIED to let
  /// portaudio determine the optimal number.
  /// * flags: Additional flags for the behaviour of the stream
  /// * callback: Some(callback) which PortAudio will call to read/write the buffers, or None
  /// when using the read and write methods
  pub fn open(input: Option<PaStreamParameters<I>>,
              output: Option<PaStreamParameters<O>>,
              sample_rate: f64,
              frames_per_buffer: u64,
              flags: PaStreamFlags,
              callback: Option<Box<StreamCallback<'a, I, O>>>)
              -> Result<Stream<'a, I, O>, PaError> {
    rportaudio::open_stream(
      input,
      output,
      sample_rate,
      frames_per_buffer,
      flags,
      callback,
    )
  }

  /// Starts the stream
  pub fn start(&self) -> PaResult {
    rportaudio::start_stream(self)
  }

  /// Stops the stream. It will block untill all audio has finished playing
  pub fn stop(&self) -> PaResult {
    rportaudio::stop_stream(self)
  }

  /// Stop stream immediately without waiting for the buffers to complete
  pub fn abort(&self) -> PaResult {
    rportaudio::abort_stream(self)
  }

  fn close(&self) -> PaResult {
    rportaudio::close_stream(self)
  }

  /// Returns wether the stream is stopped
  pub fn is_stopped(&self) -> Result<bool, PaError> {
    rportaudio::is_stream_stopped(self)
  }

  /// Returns wether the stream is active
  pub fn is_active(&self) -> Result<bool, PaError> {
    rportaudio::is_stream_active(self)
  }

  /// Get the number of frames that can be read from the stream without waiting
  pub fn num_read_available(&self) -> Result<u32, PaError> {
    rportaudio::stream_num_read_available(self)
  }

  /// Get the number of frames that can be written to the stream without waiting
  pub fn num_write_available(&self) -> Result<u32, PaError> {
    rportaudio::stream_num_write_available(self)
  }

  /// Write the given buffer to the stream. This function blocks
  ///
  /// Possible Error codes:
  ///
  /// * `CanNotWriteToAnInputOnlyStream`: when num_output_channels = 0
  /// * `BadBufferPtr`: when buffer.len() is not a multiple of num_output_channels
  /// * Some other error given by PortAudio
  pub fn write(&self, buffer: &[O]) -> PaResult {
    rportaudio::write_stream(self, buffer)
  }

  /// Reads the requested number of frames from the input devices. This function blocks until
  /// the whole buffer has been filled.
  ///
  /// Will return `CanNotReadFromAnOutputOnlyStream` if num_input_channels = 0.
  pub fn read(&self, frames: u32) -> Result<Vec<I>, PaError> {
    rportaudio::read_stream(self, frames)
  }

  /// Returns the cpu load the stream callback consumes. This will return 0.0 if the stream uses
  /// blocking read/write, or if an error occured.
  pub fn cpu_load(&self) -> f64 {
    rportaudio::stream_cpu_load(self)
  }

  /// Get the current timestamp of the stream
  pub fn time(&self) -> Duration {
    rportaudio::stream_time(self)
  }

  /// Get the actual latencies and sample rate
  ///
  /// Returns None when the stream is invalid or an error occured
  pub fn info(&self) -> Option<PaStreamInfo> {
    rportaudio::stream_info(self)
  }

  /// Set a callback which is to be called when the StreamCallback finishes
  pub fn set_finished_callback(&mut self, finished_callback: Box<StreamFinishedCallback<'a>>) -> PaResult {
    rportaudio::set_stream_finished_callback(self, finished_callback)
  }

  /// Remove any previously attached finish callback
  pub fn unset_finished_callback(&mut self) -> PaResult {
    rportaudio::unset_stream_finished_callback(self)
  }
}



/// Returns Ok when the StreamParameters are supported. This ignores the latency field.
pub fn is_format_supported<I: SampleType, O: SampleType>(input: Option<PaStreamParameters<I>>, output: Option<PaStreamParameters<O>>, sample_rate: f64) -> PaResult {
  rportaudio::is_format_supported(input, output, sample_rate)
}

