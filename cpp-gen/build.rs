use build_helper::{compile_capnp_cpp, get_capnp_file_names, workspace_root, CompileCapnpCpp};

fn main() {
    let root_dir = workspace_root();
    let capnp_files = get_capnp_file_names(&format!("{root_dir}/capnp"));

    for file_path in capnp_files.into_iter().map(|f| format!("capnp/{f}")) {
        compile_capnp_cpp(&CompileCapnpCpp {
            working_dir: Some("../c++".to_owned()),
            file_path,
            stripped_prefix: Some("capnp".to_owned()),
        });
    }
}
