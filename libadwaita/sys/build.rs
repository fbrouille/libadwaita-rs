// Generated by gir (https://github.com/gtk-rs/gir @ c88b69265102)
// from
// from gir-files (https://github.com/gtk-rs/gir-files.git @ c23f21f51d54)
// DO NOT EDIT

#[cfg(not(docsrs))]
use std::process;

#[cfg(docsrs)]
fn main() {} // prevent linking libraries to avoid documentation failure

#[cfg(not(docsrs))]
fn main() {
    if let Err(s) = system_deps::Config::new().probe() {
        println!("cargo:warning={s}");
        process::exit(1);
    }
}
