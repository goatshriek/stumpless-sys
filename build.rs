use bindgen;

fn main() {
    // enabling requested features
    let journald_enabled = if cfg!(feature = "journald") {
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
        .define("ENABLE_WINDOWS_EVENT_LOG_TARGETS", wel_enabled)
        .build();

    println!(
        "cargo:rustc-link-search=native={}/lib",
        stumpless_out.display()
    );
    println!("cargo:rustc-link-lib=stumpless");

    let bindings_builder = bindgen::Builder::default()
        .header(format!("{}/include/stumpless.h", stumpless_out.display()))
        .clang_arg(format!("-I{}/include/", stumpless_out.display()))
        .parse_callbacks(Box::new(bindgen::CargoCallbacks));

    let bindings = bindings_builder
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file("src/bindings.rs")
        .expect("Couldn't write bindings!");
}
