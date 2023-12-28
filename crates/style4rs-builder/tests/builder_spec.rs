use style4rs_builder::*;

use std::env;
use std::fs;
use std::path::Path;

#[test]
fn builder_creates_main_css_in_target_folder() {
    let manifest = env::var_os("CARGO_MANIFEST_DIR").expect("Expected $env::CARGO_MANIFEST_DIR");
    let sample_path = Path::new(&manifest).join("tests").join("samples").display().to_string();

    let folder = env::temp_dir().join("style4rs-test");
    let folder_s = folder.display().to_string();

    let result = Style4rsBuilder::build_using(Some(sample_path), Some(folder_s));
    assert!(result.is_ok());

    let main = folder.join("style4rs").join("main.css");
    let content = String::from_utf8(fs::read(main).unwrap()).unwrap();
    assert!(content.contains("This file was generated using Style4rsBuilder"));

    fs::remove_dir_all(&folder).unwrap();
}
