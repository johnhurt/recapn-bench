use std::env;
use std::fs::read_dir;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitStatus};
use std::str::from_utf8;

fn compile_capnp_cpp(file: &str) {
    let r = Command::new("capnp")
        .current_dir("c++")
        .arg("compile")
        .arg("-oc++")
        .arg(format!("capnp/{file}"))
        .arg("--src-prefix=capnp")
        .output()
        .expect("failed to execute process");

    assert!(
        r.status.success(),
        "C++ capnp failed with: {}",
        from_utf8(&r.stderr).unwrap()
    );

    // panic!("{r:?}");
}

fn get_capnp_file_names() -> Vec<String> {
    read_dir("capnp")
        .unwrap()
        .map(Result::unwrap)
        .filter(|f| f.file_type().unwrap().is_file())
        .map(|f| f.file_name().to_str().unwrap().to_owned())
        .filter(|f| f.ends_with(".capnp"))
        .collect()
}

fn main() {
    println!("cargo:rerun-if-changed=wrapper.h");

    let root_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    let cpp_files = ["addressbook.capnp.c++", "lib.cpp"]
        .into_iter()
        .inspect(|f| println!("cargo:rerun-if-changed={f}"))
        .map(|p| format!("{root_dir}/c++/{p}"));

    let capnp_files = get_capnp_file_names();

    capnp_files.iter().for_each(|f| {
        println!("cargo:rerun-if-changed={root_dir}/capnp/{f}");
    });

    capnp_files
        .iter()
        .map(String::as_str)
        .for_each(compile_capnp_cpp);

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    cc::Build::new()
        .cpp(true)
        .compiler("g++")
        .files(cpp_files)
        .compile("recapn_cpp_bench");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg("-xc++")
        .clang_arg("-Ic++")
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
