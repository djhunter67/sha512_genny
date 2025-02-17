fn main() {
    println!("cargo:rerun-if-changed=tweetnacl.c");
    println!("cargo:rerun-if-changed=tweetnacl.h");

    let bindings = bindgen::builder()
        .header("tweetnacl.h")
        .generate()
        .unwrap_or_else(|err| panic!("Failed to generate bindings: {err:?}"));

    // let out_path = std::env::var("OUT_DIR").unwrap();
    let out_path = "./";
    let out_path = std::path::Path::new(&out_path);

    bindings
        .write_to_file(out_path.join("tweetnacl_bindings.rs"))
        .unwrap_or_else(|err| {
            println!("Failed to create the 'tweetnacl_bindings.rs' C header file: {err:?}")
        });
}
