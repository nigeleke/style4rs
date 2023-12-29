use style4rs::*;

#[test]
fn it_can_be_invoked() {
    let class_name = style_sheet!("tests/samples/basics.css");
    assert!(class_name.starts_with("rcn-"));
}

fn the_class_name() -> String {
    let class_name = style_sheet!("tests/samples/basics.css");
    class_name.to_string()
}

#[test]
fn class_name_is_deterministic() {
    let class_name = style_sheet!("tests/samples/basics.css");
    assert_eq!(class_name, the_class_name());
}
