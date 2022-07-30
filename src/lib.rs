#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!("bindings.rs");

use std::fs::File;
use std::io::prelude::*;

pub fn write_message_resource_file(path: &str) -> std::io::Result<()> {
    let mut file = File::create(&path)?;
    file.write_all(b"hello!")?;
    Ok(())
}
