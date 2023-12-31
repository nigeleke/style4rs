// These samples follow the style4rs macro tests in the main style4rs crate.
// These are picked up during the testing of the Builder. Thos enables
// gathering of coverage stats, which isn't possible during the compile-time
// tests in style4rs.
//
// Note: The module isn't actually compiled, but it _is_ processed
// by Builder;:build().
//

pub fn sample_basic() {
    let (_, _) = style_sheet_str!("style4rs-builder/tests/samples/src/samples/basics.css");
}

pub fn sample_relation() {
    let (_, _) = style_sheet_str!("style4rs-builder/tests/samples/src/samples/relations.css");
}

fn sample_pseudo() {
    let (_, _) = style_sheet_str!("style4rs-builder/tests/samples/src/samples/pseudo.css");
}

fn sample_at_rules() {
    let (_, _) = style_sheet_str!("style4rs-builder/tests/samples/src/samples/at_rules.css");
}

fn sample_special_at_rules() {
    let (_, _) = style_sheet_str!("style4rs-builder/tests/samples/src/samples/special_at_rules.css");
}

fn sample_custom_pseudo_class() {
    let (_, _) = style_sheet_str!("style4rs-builder/tests/samples/src/samples/custom_pseudo.css");
}
