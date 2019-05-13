rportaudio
===


PortAudio bindings for Rust

See http://portaudio.com/

# Example

```rust
fn demo() -> rportaudio::rpa_error::PaResult {
  let stream = rportaudio::stream::Stream::open_default(
    0, // input channels
    1, // output channels
    44100.0, // sample rate
    rportaudio::stream::FRAMES_PER_BUFFER_UNSPECIFIED,
    None, // no callback
  )?;

  stream.start()?;

  let mut phase = 0.0f32;
  let mut buffer = Vec::with_capacity(44100);
  for _i in (0..44100) {
    // Small amplitude such that the test does not produce sound
    buffer.push(phase * 0.001);

    phase += 0.03;
    if phase > 1.0 { phase -= 2.0; }
  }

  stream.write(&buffer)?;

  Ok(())
}

fn main() {
  rportaudio::initialize().unwrap();
  println!("{:?}", demo());
  rportaudio::terminate().unwrap();
}
```

# Other

rportaudio crate will auto compile or find system portaudio lib, if don't want this, you can set a `PA_LINK=false` environment value cancel this action

