use build_helper::{compile_recapn, get_capnp_file_names, out_path};
use std::fs::File;
use std::io::Write;

fn main() {
    println!("cargo::rerun-if-changed=capnp");
    let capnp_files = get_capnp_file_names("capnp");

    let out_dir = out_path();
    let mut bindings = String::new();

    for capnp_file in capnp_files {
        let rs_module = capnp_file.trim_end_matches(".capnp");
        compile_recapn(format!("capnp/{capnp_file}"));

        let rs_file = format!("{}.capnp.rs", rs_module);
        bindings += &format!(
            "pub mod {rs_module}_capnp {{\n    include!(concat!(env!(\"OUT_DIR\"), \"/capnp/{rs_file}\"));\n}}\n"
        );
    }

    let mut path = out_dir.clone();
    path.push("bindings.rs");
    let mut output = File::create(path).unwrap();
    write!(output, "{}", bindings).unwrap();
}
