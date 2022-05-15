use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let mut config: cbindgen::Config = Default::default();
    config.language = cbindgen::Language::C;
    cbindgen::generate_with_config(&crate_dir, config)
        .unwrap()
        .write_to_file("include/sgs.h");

    let sgs_header_filepath = Path::new(&crate_dir).join("include/sgs.h");
    assert!(sgs_header_filepath.exists());

    // replace opaque structs with void
    let sgs_header_content = fs::read_to_string(&sgs_header_filepath).unwrap();
    let sgs_header_content = sgs_header_content.replace("FachadaWSSGSSoapBinding", "void");
    fs::write(sgs_header_filepath, sgs_header_content).unwrap();
}
