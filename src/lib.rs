#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(feature = "wel")]
use std::fs::File;
#[cfg(feature = "wel")]
use std::io::Write;
#[cfg(feature = "wel")]
use std::path::Path;

#[cfg(feature = "wel")]
pub fn write_default_events_bin_file<P: AsRef<Path>>(path: &P) -> std::io::Result<()> {
    let resource_bytes = include_bytes!(env!("STUMPLESS_DEFAULT_EVENTS_BIN_PATH"));
    let mut file = File::create(&path)?;
    file.write_all(resource_bytes).expect("couldn't write to the file!");
    Ok(())
}

#[cfg(feature = "wel")]
pub fn write_default_events_resource_file<P: AsRef<Path>>(path: &P) -> std::io::Result<()> {
    let resource_bytes = include_bytes!(env!("STUMPLESS_DEFAULT_EVENTS_RC_PATH"));
    let mut file = File::create(&path)?;
    file.write_all(resource_bytes).expect("couldn't write to the file!");
    Ok(())
}
