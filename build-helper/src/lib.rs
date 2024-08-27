use std::env;
use std::fs::read_dir;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::str::{from_utf8, FromStr};

pub struct CompileCapnpCpp {
    pub working_dir: Option<String>,
    pub file_path: String,
    pub stripped_prefix: Option<String>,
}

pub fn compile_capnp_cpp(instr: &CompileCapnpCpp) {
    let mut cmd = Command::new("capnp");

    if let Some(wd) = instr.working_dir.as_ref() {
        cmd.current_dir(wd);
    }

    cmd.arg("compile").arg("-oc++").arg(&instr.file_path);

    if let Some(prefix) = instr.stripped_prefix.as_ref() {
        cmd.arg(format!("--src-prefix={prefix}"));
    }
    let output = cmd.output().expect("failed to execute process");

    rerun_if_changed(&instr.file_path);

    assert!(
        output.status.success(),
        "C++ capnp failed with: {}",
        from_utf8(&output.stderr).unwrap()
    );
}

pub fn compile_capnp_rust_old(path: impl AsRef<str>) {
    let path = PathBuf::from_str(path.as_ref()).unwrap();
    capnpc::CompilerCommand::new().file(path).run().unwrap();
}

pub fn compile_recapn(path: impl AsRef<Path>) {
    recapnc::CapnpCommand::new()
        .file(path.as_ref())
        .write_to_out_dir();
}

pub fn get_capnp_file_names(dir: &str) -> Vec<String> {
    read_dir(dir)
        .unwrap()
        .map(Result::unwrap)
        .filter(|f| f.file_type().unwrap().is_file())
        .map(|f| f.file_name().to_str().unwrap().to_owned())
        .filter(|f| f.ends_with(".capnp"))
        .collect()
}

pub fn workspace_root() -> String {
    format!("{}/..", env::var("CARGO_MANIFEST_DIR").unwrap())
}

pub fn rerun_if_changed(path: &str) {
    println!("cargo:rerun-if-changed={path}");
}

pub fn compile_cpp_lib_dir(path: &str, lib_name: &str) {
    let cpp_files = read_dir(path)
        .unwrap()
        .map(Result::unwrap)
        .filter(|f| f.file_type().unwrap().is_file())
        .inspect(|p| rerun_if_changed(p.path().to_str().unwrap()))
        .map(|f| (f.file_name().to_str().unwrap().to_owned(), f))
        .filter(|(f, _)| f.ends_with(".cpp") || f.ends_with(".c++"))
        .map(|(_, p)| p.path())
        .collect::<Vec<_>>();

    println!("cargo::rustc-link-search=/usr/local/lib");
    println!("cargo::rustc-link-lib=static=capnp");
    println!("cargo::rustc-link-lib=static=kj");

    cc::Build::new()
        .cpp(true)
        .compiler("g++")
        .files(&cpp_files)
        .flag("-lkj")
        .emit_rerun_if_env_changed(false)
        .compile(lib_name);
}

pub fn out_path() -> PathBuf {
    PathBuf::from(env::var("OUT_DIR").unwrap())
}

pub struct Bindgen {
    pub cpp_path: String,
    pub header_file: String,
    pub output_file: String,
}

pub fn bindgen(instr: &Bindgen) {
    let out_path = out_path();

    let bindings = bindgen::Builder::default()
        .header(&instr.header_file)
        .clang_arg("-xc++")
        .clang_arg(format!("-I{}", instr.cpp_path))
        .generate()
        .expect("Unable to generate bindings");

    rerun_if_changed(&instr.header_file);

    bindings
        .write_to_file(out_path.join(&instr.output_file))
        .expect("Couldn't write bindings!");
}
