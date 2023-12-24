use style4rs_test::the_class_name;

use std::env;
use std::fs;
use std::path::Path;

#[test]
#[cfg(not(tarpaulin))]
fn macro_class_names_are_in_the_resultant_main_css() {
    // Note: the target main.css was created by the build process for the dummy lib, prior to executung these tests.
    //       OUT_DIR is not available during tarpaulin tests.
    let target_path = env::var_os("OUT_DIR").expect("Expected $env::OUT_DIR");
    let target_path = Path::new(&target_path)
        .join("style4rs")
        .join("main.css");
    let main_css = fs::read_to_string(target_path).unwrap();
    let styled_class_name = &the_class_name();
    assert!(main_css.find(styled_class_name).is_some());
}
