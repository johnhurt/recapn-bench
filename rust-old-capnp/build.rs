use std::{fs::File, io::Write};

use build_helper::{compile_capnp_rust_old, get_capnp_file_names, out_path};

fn main() {
    let capnp_files = get_capnp_file_names("../capnp");

    let out_dir = out_path();
    let mut bindings = String::new();

    for capnp_file in capnp_files {
        let rs_module = capnp_file.trim_end_matches(".capnp");
        compile_capnp_rust_old(format!("capnp/{capnp_file}"));

        let rs_file = format!("{}_capnp.rs", rs_module);
        bindings += &format!(
            "pub mod {rs_module}_capnp {{\n    include!(concat!(env!(\"OUT_DIR\"), \"/capnp/{rs_file}\"));\n}}\n"
        );
    }

    let mut path = out_dir.clone();
    path.push("bindings.rs");
    let mut output = File::create(path).unwrap();
    write!(output, "{}", bindings).unwrap();
}
