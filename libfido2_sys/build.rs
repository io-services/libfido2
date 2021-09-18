extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    if cfg!(target_os = "windows") {
        let lib_dir = std::env::var("FIDO2_LIB_DIR")
            .expect("Please set the FIDO2_LIB_DIR environment variable");
        println!("cargo:rustc-link-search=native={}", lib_dir);
        println!("cargo:rustc-link-lib=static=fido2");
    } else if cfg!(target_os = "linux") {
        println!("cargo:rustc-link-lib=dylib=fido2");
    } else if cfg!(target_os = "macos") {
        println!("cargo:rustc-link-lib=static=fido2");
    } else {
        panic!("Unsupported platform");
    }

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=wrapper.h");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Keep only the scoped-in definitions
        .allowlist_function("(?i)^fido_.*|(?i)^ctap_.*|(?i)^u2f_.*|(?i)^cose_.*|(?i)^.*es256_pk.*|(?i)^.*rs256_pk.*|(?i)^.*eddsa_pk.*")
        .allowlist_type("(?i)^fido_.*|(?i)^ctap_.*|(?i)^u2f_.*|(?i)^cose_.*|(?i)^.*es256_pk.*|(?i)^.*rs256_pk.*|(?i)^.*eddsa_pk.*")
        .allowlist_var("(?i)^fido_.*|(?i)^ctap_.*|(?i)^u2f_.*|(?i)^cose_.*|(?i)^.*es256_pk.*|(?i)^.*rs256_pk.*|(?i)^.*eddsa_pk.*")
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
