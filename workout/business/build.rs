// The client of timeline's InternalService. The proto is the mirror kept in the integration crate
// (tool/check_proto_sync.sh keeps it identical to timeline's source).
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let protoc = protoc_bin_vendored::protoc_bin_path()?;
    // SAFETY: build scripts are single-threaded during initialization.
    unsafe {
        std::env::set_var("PROTOC", protoc);
    }
    let dir = std::path::PathBuf::from(std::env::var("CARGO_MANIFEST_DIR")?).join("../integration/proto");
    let proto = dir.join("timeline/internal.proto");
    println!("cargo:rerun-if-changed={}", proto.display());
    tonic_prost_build::configure().compile_protos(&[proto], &[dir])?;
    Ok(())
}
