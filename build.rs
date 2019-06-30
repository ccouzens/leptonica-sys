extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-lib=lept");

    let mut builder = bindgen::Builder::default().header("wrapper.h");
    for line in include_str!("functions.h").lines() {
        if !line.starts_with("LEPT_DLL extern ") {
            continue;
        }
        if let Some(function_name) = line
            .split_whitespace()
            .take_while(|word| word != &"(")
            .last()
        {
            builder = builder.whitelist_function(function_name);
        }
    }
    builder = builder.whitelist_var("L_.*");
    let bindings = builder.generate().expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
