
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let protoc = protoc_bin_vendored::protoc_bin_path()?;
    // SAFETY: build scripts are single-threaded during initialization here and need
    // to set PROTOC for tonic/prost code generation in Rust 2024 edition.
    unsafe {
        std::env::set_var("PROTOC", protoc);
    }

    let manifest_dir = std::path::PathBuf::from(std::env::var("CARGO_MANIFEST_DIR")?);
    let proto_dir = manifest_dir.join("proto");
    let proto_files = vec![
        proto_dir.join("business_profile.proto"),
        proto_dir.join("business_profile_address.proto"),
        proto_dir.join("country.proto"),
        proto_dir.join("exercise.proto"),
        proto_dir.join("friend.proto"),
        proto_dir.join("person.proto"),
        proto_dir.join("person_address.proto"),
        proto_dir.join("person_info.proto"),
        proto_dir.join("team_member.proto"),
        proto_dir.join("settings.proto"),
        proto_dir.join("user.proto"),
        proto_dir.join("workout.proto"),
        proto_dir.join("resource.proto"),
    ];

    for proto in &proto_files {
        println!("cargo:rerun-if-changed={}", proto.display());
    }

    tonic_prost_build::configure().compile_protos(&proto_files, &[proto_dir])?;

    Ok(())
}

