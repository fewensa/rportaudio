use std::time::Duration;

use crate::raw_portaudio;
use crate::rpa_error::{PaError, PaResult};

pub fn to_pa_result(code: i32) -> PaResult {
  if code == raw_portaudio::paNoError {
    return Ok(());
  }
  Err(PaError::from_i32(code))
}

pub fn pa_time_to_duration(input: f64) -> Duration {
  let valid_input = if input < 0.0 { 0.0 } else { input };
  let secs = valid_input.floor();
  let nanos = (valid_input - secs) * 1e9;
  Duration::new(secs as u64, nanos as u32)
}

pub fn duration_to_pa_time(duration: Duration) -> f64 {
  duration.as_secs() as f64 + (duration.subsec_nanos() as f64 * 1e-9)
}

#[cfg(test)]
mod test {
  #[test]
  fn test_conversion() {
    let seconds = 2.512389131321938123681627;
    let duration = super::pa_time_to_duration(seconds);
    let seconds2 = super::duration_to_pa_time(duration);

    println!("{}", (seconds - seconds2).abs());
    assert!((seconds - seconds2).abs() <= 1e-8);

    let duration2 = super::pa_time_to_duration(-seconds);
    let seconds3 = super::duration_to_pa_time(duration2);

    println!("{}.{}", duration2.as_secs(), duration2.subsec_nanos());
    assert_eq!(duration2.as_secs(), 0);
    assert_eq!(duration2.subsec_nanos(), 0);
    println!("{}", seconds3.abs());
    assert!(seconds3.abs() <= 1e-8);
  }
}
