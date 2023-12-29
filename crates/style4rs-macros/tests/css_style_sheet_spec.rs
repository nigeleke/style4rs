//! Tests attributed to [Abishek P](https://github.com/abishekatp) from the [stylers](https://github.com/abishekatp/stylers) repository.
//!

use style4rs_macros::*;

// Note: to check why test cases are failing we can use the text compare tools to compare and see the differences between left and right values of a test case
// Ref: https://www.w3schools.com/cssref/css_selectors.php
//
#[test]
pub fn basic() {
    let (class_name, style) = style_sheet_str!("tests/samples/basics.css");

    let expected = format!(".two.{}{{color:#ff0}}.two.one.{}{{color:#ff0}}.two.{} .one.{}{{color:#ff0}}.two.{},.one.{}{{color:#ff0}}#firstname.{}{{background-color:#ff0}}.{}{{background-color:#ff0}}div.{}{{border:1px solid #000;margin:25px 50px 75px 100px;background-color:#add8e6}}div.{} .one.{} p.{}{{color:#00f}}div.one.{} p.{} div.{}{{color:#00f}}div.{} #two.{}{{color:#00f}}h2.{},a.{}{{color:purple}}",
        class_name, class_name, class_name, class_name, class_name,
        class_name, class_name, class_name, class_name, class_name,
        class_name, class_name, class_name, class_name, class_name,
        class_name, class_name, class_name, class_name);

        assert_eq!(style, expected);
}

#[test]
pub fn relation() {
    let (class_name, style) = style_sheet_str!("tests/samples/relations.css");

    let expected = format!(r#"div.{}>p.{}{{background-color:#ff0}}div.{}+p.{}{{background-color:#ff0}}p.{}~ul.{}{{background:red}}a[target].{}{{background-color:#ff0}}a[title="I am ,testing"].{}{{background-color:#ff0}}[title~=flower].{}{{background-color:#ff0}}[lang|=en].{}{{background-color:#ff0}}div[class^=test].{}{{background-color:#ff0}}div[class$=test].{}{{background-color:#ff0}}div.{} [class$=test].{}{{background-color:#ff0}}div[class*=test].{}{{background-color:#ff0}}"#,
        class_name, class_name, class_name, class_name, class_name, 
        class_name, class_name, class_name, class_name, class_name,
        class_name, class_name, class_name, class_name, class_name);

    assert_eq!(style, expected);
}

#[test]
fn pseudo() {
    // Difference from [stylers](https://github.com/abishekatp/stylers)
    // Namespaces do not have custom class appended.
    let (class_name, style) = style_sheet_str!("tests/samples/pseudo.css");

    let expected = format!(r#".one.{}:hover{{background-color:green}}p.{}:before{{content:"Read this: "}}div.{}:nth-child(2){{background-color:green}}p.{}:lang(it){{background:#ff0}}svg|a.{}{{background-color:#f0f8ff}}.{}:not(body){{background:red}}:root{{--blue:#1e90ff}}body.{}{{background-color:var(--blue)}}#container.{}{{--first-color:#290}}#thirdParagraph.{}{{background-color:var(--first-color);color:var(--second-color)}}"#,
    class_name, class_name, class_name, class_name, class_name,
    class_name, class_name, class_name, class_name);
    
    assert_eq!(style, expected);
}

#[test]
fn at_rules() {
    // From: lightningcss:
    // @charset is removed by rust-cssparser if it’s the first rule in the stylesheet.
    // Anything left is technically invalid, however, users often concatenate CSS files
    // together, so we are more lenient and simply ignore @charset rules in the middle of a file.
    //
    let (class_name, style) = style_sheet_str!("tests/samples/at_rules.css");

    let expected = format!(r#"@import "landscape.css" screen and (orientation:landscape);@namespace svg "http://www.w3.org/2000/svg";@layer theme,layout,utilities;@supports (display:flex){{@media screen and (width>=900px){{article.{}{{display:flex}}}}}}@supports (display:flex){{.flex-container.{}>.{}{{text-shadow:0 0 2px #00f;float:none}}.flex-container.{}{{display:flex}}}}@document url(https://www.example.com/){{h1 {{ color: green; }}}}@layer framework{{@layer layout{{p.{}{{margin-block:1rem;font:.9em/1.2 Arial,Helvetica,sans-serif;content:"hello";content:"hello"}}}}}}"#,
        class_name, class_name, class_name, class_name, class_name);

    assert_eq!(style, expected);
}

#[test]
fn special_at_rules() {
    let (_, style) = style_sheet_str!("tests/samples/special_at_rules.css");

    let expected = r#"@page{size:A4;margin:10%;@top-left-corner{content:"Page " counter(page)}}@font-face{font-family:Trickster;src:local(Trickster),url(trickster-COLRv1.otf)format("opentype")tech(color-colrv1),url(trickster-outline.otf)format("opentype"),url(trickster-outline.woff)format("woff")}@keyframes spin1{to{-webkit-transform:rotate(360deg)}}@-webkit-keyframes spin2{to{-webkit-transform:rotate(360deg)}}@counter-style thumbs{system:cyclic;symbols:"👍";suffix:" "}@font-feature-values Font One{@styleset { nice-style: 12; }}@property --property-name{syntax:"<color>";inherits:false;initial-value:#c0ffee}"#;

    assert_eq!(style, expected);
}

#[test]
fn custom_pseudo_class() {
    let (class_name, style) = style_sheet_str!("tests/samples/custom_pseudo.css");

    let expected = format!(r#":deep(h3 div){{color:orange}}div.{} :deep(h3){{color:orange}}div.{}>:deep(h3){{color:orange}}"#,
        class_name, class_name);

    assert_eq!(style, expected);
}
