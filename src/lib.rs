//! FFI bindings for the Stumpless logging library.
//!
//! These wrappings are intended to be the bare minimum to use Stumpless from
//! a Rust application.
//!
//!
//! # Create Features
//! Stumpless provides a number of build configuration options that can be used
//! to enable or disable different functionality. This crate exposes these
//! options as the following features.
//!
//!
//! ### Target Features
//!
//! * **journald** -
//!   Enables targets that can send logs to a systemd journald daemon.
//! * **network** -
//!   Enables targets that can send logs to a server over a network connection.
//! * **socket** -
//!   Enables targets that can send logs to Unix sockets.
//! * **wel** -
//!   Enables targets that can send logs to the Windows Event Log.

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

/// Writes a compiled resource file (.bin) to the provided file.
#[cfg(feature = "wel")]
pub fn write_default_events_bin_file<P: AsRef<Path>>(path: &P) -> std::io::Result<()> {
    let resource_bytes = include_bytes!(env!("STUMPLESS_DEFAULT_EVENTS_BIN_PATH"));
    let mut file = File::create(&path)?;
    file.write_all(resource_bytes).expect("couldn't write to the file!");
    Ok(())
}

/// Writes the resource definition (.rc) file for the default Stumpless events
/// to the provided file.
#[cfg(feature = "wel")]
pub fn write_default_events_resource_file<P: AsRef<Path>>(path: &P) -> std::io::Result<()> {
    let resource_bytes = include_bytes!(env!("STUMPLESS_DEFAULT_EVENTS_RC_PATH"));
    let mut file = File::create(&path)?;
    file.write_all(resource_bytes).expect("couldn't write to the file!");
    Ok(())
}
