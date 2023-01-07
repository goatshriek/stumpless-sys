use std::fs;

use std::env;
use std::path::PathBuf;

fn main() {
    // enabling requested features
    let journald_enabled = if cfg!(feature = "journald") {
        "ON"
    } else {
        "OFF"
    };
    let network_enabled = if cfg!(feature = "network") {
        "ON"
    } else {
        "OFF"
    };
    let socket_enabled = if cfg!(feature = "socket") {
        "ON"
    } else {
        "OFF"
    };
    let wel_enabled = if cfg!(feature = "wel") {
        "ON"
    } else {
        "OFF"
    };

    let stumpless_out = cmake::Config::new("stumpless")
        .define("BUILD_SHARED_LIBS", "OFF")
        .define("ENABLE_JOURNALD_TARGETS", journald_enabled)
        .define("ENABLE_NETWORK_TARGETS", network_enabled)
        .define("ENABLE_SOCKET_TARGETS", socket_enabled)
        .define("ENABLE_WINDOWS_EVENT_LOG_TARGETS", wel_enabled)
        .build();

    println!(
        "cargo:rustc-link-search=native={}/lib",
        stumpless_out.display()
    );
    println!("cargo:rustc-link-lib=stumpless");
    if cfg!(feature = "journald") {
        println!("cargo:rustc-link-lib=systemd");
    }
    if cfg!(feature = "wel") {
        println!("cargo:rustc-link-lib=ktmw32");
    }

    let bindings_builder = bindgen::Builder::default()
        .header(format!("{}/include/stumpless.h", stumpless_out.display()))
        .clang_arg(format!("-I{}/include/", stumpless_out.display()))
        .allowlist_function("stump.*")
        .allowlist_type("stump.*")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks));

    let bindings = bindings_builder
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    if cfg!(feature = "wel") {
        fs::copy(stumpless_out.join("build").join("default_events.rc"), "src/default_events.rc").expect("Couldn't copy default events resource!");
        fs::copy(stumpless_out.join("build").join("default_events_MSG00409.bin"), "src/default_events_MSG00409.bin").expect("Couldn't copy default events binary!");
    }
}
