fn main() -> Result<(), Box<dyn std::error::Error>> {
    let protoc = protoc_bin_vendored::protoc_bin_path()?;
    // SAFETY: build scripts are single-threaded during initialization here and need
    // to set PROTOC for tonic/prost code generation in Rust 2024 edition.
    unsafe {
        std::env::set_var("PROTOC", protoc);
    }

    let manifest_dir = std::path::PathBuf::from(std::env::var("CARGO_MANIFEST_DIR")?);
    let proto_dir = manifest_dir.join("proto");
    // The timeline's own contract. A new proto in the folder must trigger the build too, not only
    // edits of known files.
    let timeline_dir = proto_dir.join("timeline");
    println!("cargo:rerun-if-changed={}", timeline_dir.display());
    let mut proto_files = Vec::new();
    for entry in std::fs::read_dir(&timeline_dir)? {
        let path = entry?.path();
        if path.extension().is_some_and(|ext| ext == "proto") {
            proto_files.push(path);
        }
    }
    proto_files.sort();
    for proto in &proto_files {
        println!("cargo:rerun-if-changed={}", proto.display());
    }

    tonic_prost_build::configure().compile_protos(&proto_files, &[proto_dir])?;

    Ok(())
}
