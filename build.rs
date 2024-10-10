extern crate bindgen;

use std::env;
use std::path::absolute;
use std::path::PathBuf;

static MISSING_PATH_CONFIG_MSG: &str =
    "The `bsec_include_path` configuration option must be set to the \
path of the BSEC header files. You can do that by creating a \
`.cargo/config` file in your crate with the following content:

[build]
rustflags = [
    '--cfg', 'bsec_include_path=\"/path/to/bsec/includes\"',
    '--cfg', 'bsec_library_path=\"/path/to/bsec/includes\"',
]
";

static BINDINGS_WRITE_ERR_MSG: &str = "Could not write BSEC bindings.";

fn main() {
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join("bsec.rs");

    if let Ok(_) = std::env::var("DOCS_RS") {
        std::fs::File::create(out_path).expect(BINDINGS_WRITE_ERR_MSG);
        return;
    }

    env::vars()
        .map(|(k, v)| format!("{}={}", k, v))
        .for_each(|s| eprintln!("{}", s));
    let bsec_include_path = env::var("CARGO_CFG_BSEC_INCLUDE_PATH").expect(MISSING_PATH_CONFIG_MSG);
    let bsec_library_path = env::var("CARGO_CFG_BSEC_LIBRARY_PATH").expect(MISSING_PATH_CONFIG_MSG);

    println!("cargo:rerun-if-changed={}", bsec_include_path);
    println!("cargo:rustc-link-search=native={}", bsec_library_path);
    println!("cargo:rustc-link-lib=static=algobsec");

    let bindings = bindgen::Builder::default()
        .header(
            absolute(PathBuf::from(bsec_include_path).join("bsec_interface.h"))
                .unwrap()
                .to_str()
                .unwrap(),
        )
        .use_core()
        .ctypes_prefix("cty")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate BSEC bindings.");
    bindings
        .write_to_file(out_path)
        .expect(BINDINGS_WRITE_ERR_MSG);
}
