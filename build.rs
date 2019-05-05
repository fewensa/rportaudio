extern crate bindgen;

use std::path::PathBuf;

fn main() {

  println!("cargo:rustc-link-lib=bz2");



  let bindings = bindgen::Builder::default()
    .header("pa_include/pa_linux_alsa.h")
    .enable_cxx_namespaces()
    .layout_tests(false)
    .whitelist_type("PaAlsaStreamInfo")
    .whitelist_function("PaAlsa_InitializeStreamInfo")
    .whitelist_function("PaAlsa_EnableRealtimeScheduling")
    .whitelist_function("PaAlsa_EnableWatchdog")
    .whitelist_function("PaAlsa_GetStreamInputCard")
    .whitelist_function("PaAlsa_GetStreamOutputCard")
    .whitelist_function("PaAlsa_SetNumPeriods")
    .whitelist_function("PaAlsa_SetRetriesBusy")
    .whitelist_function("PaAlsa_SetLibraryPathName")
    .generate()
    .expect("Unable to generate bindings");

  bindings
    .write_to_file(PathBuf::from("src/pa_include/pa_linux_alsa.rs").as_path())
    .expect("Couldn't write bindings!");



  let bindings = bindgen::Builder::default()
    .header("pa_include/pa_ringbuffer.h")
    .enable_cxx_namespaces()
    .layout_tests(false)
    .generate()
    .expect("Unable to generate bindings");

  bindings
    .write_to_file(PathBuf::from("src/pa_include/pa_ringbuffer.rs").as_path())
    .expect("Couldn't write bindings!");



  let bindings = bindgen::Builder::default()
    .header("pa_include/pa_util.h")
    .enable_cxx_namespaces()
    .layout_tests(false)
//    .whitelist_type("PaUtilHostApiRepresentation")
    .whitelist_function("PaUtil_SetLastHostErrorInfo")
    .whitelist_function("PaUtil_AllocateMemory")
    .whitelist_function("PaUtil_FreeMemory")
    .whitelist_function("PaUtil_CountCurrentlyAllocatedBlocks")
    .whitelist_function("PaUtil_InitializeClock")
    .whitelist_function("PaUtil_GetTime")
    .generate()
    .expect("Unable to generate bindings");

  bindings
    .write_to_file(PathBuf::from("src/pa_include/pa_util.rs").as_path())
    .expect("Couldn't write bindings!");



//  let bindings = bindgen::Builder::default()
//    .header("pa_include/portaudio.h")
//    .enable_cxx_namespaces()
//    .layout_tests(false)
//    .generate()
//    .expect("Unable to generate bindings");
//
//  bindings
//    .write_to_file(PathBuf::from("src/pa_include/portaudio.rs").as_path())
//    .expect("Couldn't write bindings!");




}
