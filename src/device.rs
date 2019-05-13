use crate::rpa_error::PaError;
use crate::rportaudio;
use crate::types::{DeviceIndex, PaDeviceInfo};

/// Retrieve the number of available devices.
pub fn count() -> Result<u32, PaError> {
  rportaudio::device_count()
}


/// Retrieve the index of the default input device
///
/// Will return None when none are available.
pub fn default_input() -> Option<DeviceIndex> {
  rportaudio::default_input_device()
}


/// Retrieve the index of the default output device
///
/// Will return None when none are available.
pub fn default_output() -> Option<DeviceIndex> {
  rportaudio::default_output_device()
}


/// Get info about a particular device
///
/// Returns None when the index is out of range.
pub fn info(index: DeviceIndex) -> Option<PaDeviceInfo> {
  rportaudio::device_info(index)
}

/// Converts a device index from a specific host API to a global device index
///
/// Returns Err(InvalidHostApi) when the host_api is out of range, and Err(InvalidDevice) when
/// host_api_device_index is out of range.
///
/// ```
/// // We retrieve the index of device 3 of api 1
/// let device_index = match rportaudio::device::hostapi_device_index_to_device_index(1, 3) {
///   Ok(n) => n,
///   Err(e) => { println!("Error: {:?}", e); return; }
/// };
/// ```
pub fn hostapi_device_index_to_device_index(hostapi: u32, hostapi_device_index: u32) -> Result<u32, PaError> {
  rportaudio::hostapi_device_index_to_device_index(hostapi, hostapi_device_index)
}

