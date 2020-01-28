extern crate bindgen;

use std::env;
use std::path::PathBuf;
#[cfg(windows)]
use vcpkg;

#[cfg(windows)]
fn find_leptonica_system_lib() -> Option<String> {
    let lib = vcpkg::Config::new().find_package("leptonica").unwrap();

    let include = lib
        .include_paths
        .iter()
        .map(|x| x.to_string_lossy())
        .collect::<String>();
    Some(include)
}

#[cfg(not(windows))]
fn find_leptonica_system_lib() -> Option<String> {
    println!("cargo:rustc-link-lib=lept");
    None
}

fn main() {
    let clang_extra_include = find_leptonica_system_lib();

    let mut bindings = bindgen::Builder::default().header("wrapper.h");

    if let Some(include_path) = clang_extra_include {
        bindings = bindings.clang_arg(format!("-I{}", include_path));
    }

    let bindings = bindings
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
