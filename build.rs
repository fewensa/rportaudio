mod portaudio_build {
  use std;
  use std::path::Path;
  use std::process::Command;

  pub fn download_sources() {
    let mut command = Command::new("cmake");
    command.arg("-P");
    command.arg("download.cmake");

    match command.status() {
      Ok(status) =>
        if !status.success() {
          panic!("Failed to execute command: {:?}", command)
        },
      Err(error) =>
        panic!("Failed to execute command: {:?}\n{}", command, error)
    }
  }

  pub fn build_sources() {
    let out_dir_env = std::env::var("OUT_DIR").unwrap();
    let out_dir = Path::new(&out_dir_env);

    let source_path = out_dir.join("portaudio");

    cmake::Config::new(source_path)
      .define("CMAKE_ARCHIVE_OUTPUT_DIRECTORY_DEBUG", out_dir)
      .define("CMAKE_ARCHIVE_OUTPUT_DIRECTORY_RELEASE", out_dir)
      .out_dir(out_dir)
      .build_target("portaudio_static")
      .build();

    let build_dir = out_dir.join("build");
    Command::new("make").arg("--directory").arg(build_dir.to_str().unwrap()).output().unwrap();

    std::fs::read_dir(out_dir).unwrap().for_each(|f| {
      println!("====> {:?}", std::fs::canonicalize(f.unwrap().path()));
    });

    println!("cargo:rustc-link-search=native={}", build_dir.to_str().unwrap());
    println!("cargo:rustc-link-lib=dylib=portaudio");
  }
}


#[cfg(not(windows))]
fn compile() {
  match pkg_config::find_library("portaudio-2.0") {
    Ok(..) => {}
    Err(e) => {
      portaudio_build::download_sources();
      portaudio_build::build_sources();
    }
  }
}

#[cfg(windows)]
fn compile() {
  portaudio_build::download_sources();
  portaudio_build::build_sources();
}


fn main() {
  let pa_compile = match std::env::var("PA_LINK") {
    Ok(pac) => {
      match &pac[..] {
        "no" | "false" => false,
        _ => true
      }
    }
    Err(_) => true
  };

  if pa_compile {
    compile();
  }
}
