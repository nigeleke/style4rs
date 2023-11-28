use style4rs_test::the_class_name;

use style4rs::*;

use std::env;
use std::fs;
use std::path::Path;

#[test]
fn it_can_be_invoked() {
    let class_name = style! {
        color: white;
        background-color: black;
    };
    assert!(class_name.starts_with("rcn-"));
}

#[test]
fn class_name_is_deterministic() {
    let class_name = style! {
    #one1{
        color: blue 6px;
    }
    div.one{
        color: red;
        content: "hello";
        font: "1.3em/1.2" Arial, Helvetica, sans-serif;
    }
    wibble {
        border: 1px solid black;
        margin: 25px 50px 75px 100px;
        background-color: lightblue;
    }
    h2 {
        color: purple;
    }
    @media only screen and (max-width: 1000px) {
        h3 {
            background-color: lightblue;
            color: blue
        }
    }
    };
    assert_eq!(class_name, the_class_name());
}

#[test]
fn macro_class_names_are_in_the_resultant_main_css() {
    // Note: the target main.css was created by the build process for the dummy lib, prior to executung these tests.
    let target_path = env::var_os("OUT_DIR").expect("Expected $env::OUT_DIR");
    let target_path = Path::new(&target_path)
        .join("style4rs")
        .join("main.css");
    let main_css = fs::read_to_string(target_path).unwrap();
    let styled_class_name = &the_class_name();
    assert!(main_css.find(styled_class_name).is_some());
}
