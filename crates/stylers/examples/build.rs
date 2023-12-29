use style4rs::Style4rsBuilder;

fn main() {
    // ++
    // See [crates/style4rs-test](crates/style4rs-test/build.rs) for an actual (simple) `build.rs`
    // The follow is NOT required as the OUT_DIR environment variable is normally defined when
    // `build.rs` gets invoked...
    //
    const OUT_DIR: &str = "OUT_DIR";

    let mut housekeep_out_dir = false;
    let folder = std::env::temp_dir().join("style4rs-examples");
    let folder_s = folder.display().to_string();

    if std::env::var_os(OUT_DIR).is_none() {
        std::env::set_var("OUT_DIR", folder_s);
        housekeep_out_dir = true;
    }
    // --


    // "Standard" `build.rs` call...
    Style4rsBuilder::build().ok();

    
    // ++
    // Housekeeping temp folder - also not required in `build.rs`.
    //
    if housekeep_out_dir {
        std::fs::remove_dir_all(&folder).unwrap();
    }
    //--
}
