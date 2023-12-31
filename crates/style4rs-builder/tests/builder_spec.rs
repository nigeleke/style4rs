use style4rs_builder::builder::*;

use std::env;
use std::fs;
use std::path::Path;

#[test]
fn builder_creates_target_file() {
    let manifest = env::var_os("CARGO_MANIFEST_DIR").expect("Expected $env::CARGO_MANIFEST_DIR");
    let sample_path = Path::new(&manifest).join("tests").join("samples");

    let test_folder = env::temp_dir().join("style4rs-test");
    let target_file = test_folder.join("style4rs").join("main.css");

    let result = Builder::new()
        .using_in_folder(sample_path.as_path())
        .using_out_file(target_file.as_path())
        .build();
    assert!(result.is_ok());

    let content = String::from_utf8(fs::read(target_file).unwrap()).unwrap();
    assert!(content.contains("This file was generated using Builder"));

    fs::remove_dir_all(&test_folder).unwrap();
}
