// Generated by gir (https://github.com/gtk-rs/gir @ 8baeaa4)
// from
// from gir-files (https://github.com/gtk-rs/gir-files.git @ 6088bb6)
// DO NOT EDIT

#[cfg(not(feature = "dox"))]
use std::process;

#[cfg(feature = "dox")]
fn main() {} // prevent linking libraries to avoid documentation failure

#[cfg(not(feature = "dox"))]
fn main() {
    if let Err(s) = system_deps::Config::new().probe() {
        println!("cargo:warning={}", s);
        process::exit(1);
    }
}
