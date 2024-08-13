use build_helper::{bindgen, compile_cpp_lib_dir, workspace_root, Bindgen};

fn main() {
    let root_dir = workspace_root();
    let cpp_path = format!("{root_dir}/c++");
    compile_cpp_lib_dir(&cpp_path, "recapn_bench_cpp");
    bindgen(&Bindgen {
        cpp_path,
        header_file: "../wrapper.h".into(),
        output_file: "bindings.rs".into(),
    });
}
