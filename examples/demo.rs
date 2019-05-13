static SECONDS: usize = 1;

fn main() {
  rportaudio::initialize().unwrap();
  print_devs();
  println!("{:?}", demo());
  rportaudio::terminate().unwrap();
}

fn print_devs() {
  for i in 0..rportaudio::device::count().unwrap() {
    match rportaudio::device::info(i) {
      None => {}
      Some(info) => println!("{}: {}", i, info.name),
    }
  }
}

fn demo() -> rportaudio::rpa_error::PaResult {
  let stream = rportaudio::stream::Stream::open_default(1, 1, 44100.0, rportaudio::stream::FRAMES_PER_BUFFER_UNSPECIFIED, None)?;

  stream.start()?;

  let input = stream.read(44100)?;

  let mut phase = 0.0f32;
  let mut buffer = Vec::with_capacity(44100 * SECONDS);
  for _i in 0..44100 * SECONDS {
    buffer.push(phase);

    phase += 0.007;
    if phase > 1.0 { phase -= 2.0; }
  }

  let waiter = std::thread::spawn(move || {
    std::thread::sleep(std::time::Duration::from_secs(SECONDS as u64));
  });

  match stream.write(&*buffer) {
    Err(e) => { println!("write 1: Err({:?})", e); }
    Ok(()) => {}
  }

  match stream.write(&*input) {
    Err(e) => { println!("write 2: Err({:?})", e); }
    Ok(()) => {}
  }

  let _ = waiter.join();

  Ok(())
}
