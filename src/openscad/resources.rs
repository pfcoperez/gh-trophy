use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "openscad/"]
struct StaticFiles;

// Unsage ungrap is used here because we
// really want to panic if these static resources can not be obtained.

pub fn trophy_without_data() -> String {
    let raw_bytes = StaticFiles::get("trophy.scad").unwrap().data.to_vec();
    return String::from_utf8(raw_bytes).unwrap();
}
