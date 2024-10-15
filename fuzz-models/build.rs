use std::env;

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    cbindgen::Builder::new()
        .with_crate(crate_dir)
        .with_header("#pragma once\nstruct AllTypesC;\n#include <cstdint>")
        .with_no_includes()
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file("fuzz_models.h");
}
