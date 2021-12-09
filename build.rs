extern crate bindgen;

#[cfg(target_os = "macos")]
use pkg_config;
use std::env;
use std::path::PathBuf;
#[cfg(windows)]
use vcpkg;

// const MINIMUM_LEPT_VERSION: &str = "1.80.0";

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

// On macOS, we sometimes need additional search paths, which we get using pkg-config
#[cfg(target_os = "macos")]
fn find_leptonica_system_lib() -> Option<String> {
    let pk = pkg_config::Config::new()
        // .atleast_version(MINIMUM_LEPT_VERSION)
        .probe("lept")
        .unwrap();
    // Tell cargo to tell rustc to link the system proj shared library.
    println!("cargo:rustc-link-search=native={:?}", pk.link_paths[0]);
    println!("cargo:rustc-link-lib=lept");

    let mut include_path = pk.include_paths[0].clone();
    // The include file used in this project has "leptonica" as part of
    // the header file already
    include_path.pop();
    Some(include_path.to_string_lossy().to_string())
}

#[cfg(target_os = "linux")]
fn find_leptonica_system_lib() -> Option<String> {
    let pk = pkg_config::Config::new()
        .probe("lept")
        .unwrap();
    // Tell cargo to tell rustc to link the system proj shared library.
    println!("cargo:rustc-link-search=native={:?}", pk.link_paths[0]);
    println!("cargo:rustc-link-lib=lept");

    let mut include_path = pk.include_paths[0].clone();
    // The include file used in this project has "leptonica" as part of
    // the header file already
    include_path.pop();
    Some(include_path.to_string_lossy().to_string())
}

#[cfg(all(not(windows), not(target_os = "macos"), not(target_os = "linux")))]
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
