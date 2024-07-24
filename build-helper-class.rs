extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    let string = env!("JAVA_HOME").to_string();
    let bindings = bindgen::builder()
        .header(string.clone() + "\\include\\jvmti.h")
        .header(string.clone() + "\\include\\win32\\jni_md.h")
        // We want jni defs from the jni-sys crate
        .raw_line("use jni_sys::*;")
        .allowlist_recursively(false)

        .allowlist_type(".*JVMTI.*")
        .allowlist_type(".*jvmti.*")
        .allowlist_type("^jlocation")
        .allowlist_type("^jthread.*")
        .allowlist_type("^jniNativeInterface$")

        // This is not defined in jni-sys for some reason
        .allowlist_type("^_?jrawMonitorID")

        .allowlist_var(".*JVMTI.*")
        .allowlist_var(".*jvmti.*")

        .derive_default(true)

        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from("D:\\RustProject\\stackparam\\src\\jvmti_sys");
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}