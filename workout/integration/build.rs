fn main() -> Result<(), Box<dyn std::error::Error>> {
	let protoc = protoc_bin_vendored::protoc_bin_path()?;
	unsafe {
		std::env::set_var("PROTOC", protoc);
	}
	let out_dir = std::path::PathBuf::from(std::env::var("OUT_DIR")?);
	let descriptor_path = out_dir.join("socialgym_descriptor.bin");

	tonic_prost_build::configure()
		.file_descriptor_set_path(descriptor_path)
		.compile_protos(
		&[
			"proto/user.proto",
			"proto/person_info.proto",
			"proto/person_address.proto",
			"proto/country.proto",
			"proto/exercise.proto",
			"proto/workout.proto",
			"proto/business_profile_address.proto",
			"proto/business_profile.proto",
			"proto/person.proto",
			"proto/friend.proto",
			"proto/team_member.proto",
			"proto/settings.proto",
		],
		&["proto"],
	)?;
	Ok(())
}