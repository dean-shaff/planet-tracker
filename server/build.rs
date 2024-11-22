use std::env;
use std::path::PathBuf;

fn main() {
    cc::Build::new()
        .file("src/astronomy.c")
        .include("src")
        .opt_level(3)
        .compile("libastronomyc");

    let bindings = bindgen::Builder::default()
        .header("src/astronomy.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
