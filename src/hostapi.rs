use crate::types::{PaHostErrorInfo, HostApiIndex, PaHostApiInfo};
use crate::rportaudio;
use crate::rpa_error::PaError;

/// Return information about the last host error encountered.
///
/// The values in this structure will only be valid if a PortAudio function has previously returned
/// the UnanticipatedHostError error code.
pub fn last_error() -> Option<PaHostErrorInfo> {
  rportaudio::last_host_error()
}


/// Get the number of host API's available
pub fn count() -> Result<u32, PaError> {
  rportaudio::hostapi_count()
}


/// Get the default Host API
pub fn default() -> Result<HostApiIndex, PaError> {
  rportaudio::default_hostapi()
}


/// Get information about a specific Host API
///
/// Returns None when an invalid index is given
pub fn info(index: HostApiIndex) -> Option<PaHostApiInfo> {
  rportaudio::hostapi_info(index)
}



