//! Tests attributed to [Abishek P](https://github.com/abishekatp) from the [stylers](https://github.com/abishekatp/stylers) repository.
//!
//! Ref: https://www.w3schools.com/cssref/css_selectors.php

use style4rs_macros::*;

#[test]
fn test_1() {
    let (class_name, style) = style_str! {.two{
            /* This comment should be ignored */
            color: yellow;
        }
    };
    let expected = format!(".two.{}{{color:#ff0}}", class_name);
    assert_eq!(style, expected);
}

#[test]
fn test_2() {
    let (class_name, style) = style_str! {
        .two.one  {
            color: yellow;
        }
    };
    let expected = format!(".two.one.{}{{color:#ff0}}", class_name);
    assert_eq!(style, expected);
}

#[test]
fn test_3() {
    let (class_name, style) = style_str! {
        .two .one{
            color: yellow;
        }
    };
    let expected = format!(".two.{} .one.{}{{color:#ff0}}", class_name, class_name);
    assert_eq!(style, expected);
}

#[test]
fn test_4() {
    let (class_name, style) = style_str! {
        #firstname{
            background-color: yellow;
        }
    };
    let expected = format!("#firstname.{}{{background-color:#ff0}}", class_name);
    assert_eq!(style, expected);
}

#[test]
fn test_5() {
    let (class_name, style) = style_str! {
        *{
            background-color: yellow;
        }
    };
    let expected = format!(".{}{{background-color:#ff0}}", class_name);
    assert_eq!(style, expected);
}

#[test]
fn test_6() {
    let (class_name, style) = style_str! {
        div{
            border: 1px solid black;
            margin: 25px 50px 75px 100px;
            background-color: lightblue;
        }
    };
    let expected = format!("div.{}{{border:1px solid #000;margin:25px 50px 75px 100px;background-color:#add8e6}}", class_name);
    assert_eq!(style, expected);
}

#[test]
fn test_7() {
    let (class_name, style) = style_str! {
        div .one p{
            color: blue;
        }
    };
    let expected = format!("div.{} .one.{} p.{}{{color:#00f}}", class_name, class_name, class_name);
    assert_eq!(style, expected);
}

#[test]
fn test_8() {
    let (class_name, style) = style_str! {
        div.one p div{
            color: blue;
        }
    };
    let expected = format!("div.one.{} p.{} div.{}{{color:#00f}}", class_name, class_name, class_name);
    assert_eq!(style, expected);
}

#[test]
fn test_9() {
    let (class_name, style) = style_str! {
        div #two{
            color: blue;
        }
    };
    let expected = format!("div.{} #two.{}{{color:#00f}}", class_name, class_name);
    assert_eq!(style, expected);
}

#[test]
fn test_10() {
    let (class_name, style) = style_str! {
        h2 , a{
            color: purple;
        }
    };
    let expected = format!("h2.{},a.{}{{color:purple}}", class_name, class_name);
    assert_eq!(style, expected);
}

#[test]
fn test_11() {
    let (class_name, style) = style_str! {
        div > p{
            background-color: yellow;
        }
    };
    let expected = format!("div.{}>p.{}{{background-color:#ff0}}", class_name, class_name);
    assert_eq!(style, expected);
}

#[test]
fn test_12() {
    let (class_name, style) = style_str! {
        div + p {
            background-color: yellow;
        }
    };
    let expected = format!("div.{}+p.{}{{background-color:#ff0}}", class_name, class_name);
    assert_eq!(style, expected);
}

#[test]
fn test_13() {
    let (class_name, style) = style_str! {
        p ~ ul {
            background: #ff0000;
        }
    };
    let expected = format!("p.{}~ul.{}{{background:red}}", class_name, class_name);
    assert_eq!(style, expected);
}

#[test]
fn test_14() {
    let (class_name, style) = style_str! {
        a[target] {
            background-color: yellow;
        }
    };
    let expected = format!("a[target].{}{{background-color:#ff0}}", class_name);
    assert_eq!(style, expected);
}

#[test]
fn test_15() {
    let (class_name, style) = style_str! {
        a[title="I am ,testing"] {
            background-color: yellow;
        }
    };
    let expected = format!("a[title=\"I am ,testing\"].{}{{background-color:#ff0}}", class_name);
    assert_eq!(style, expected);
}

#[test]
fn test_16() {
    let (class_name, style) = style_str! {
        [title~=flower] {
            background-color: yellow;
        }
    };
    let expected = format!("[title~=flower].{}{{background-color:#ff0}}", class_name);
    assert_eq!(style, expected);
}

#[test]
fn test_17() {
    let (class_name, style) = style_str! {
        [lang|=en] {
            background-color: yellow;
        }
    };
    let expected = format!("[lang|=en].{}{{background-color:#ff0}}", class_name);
    assert_eq!(style, expected);
}

#[test]
fn test_18() {
    let (class_name, style) = style_str! {
        div[class^="test"] {
            background-color: yellow;
        }
    };
    let expected = format!("div[class^=test].{}{{background-color:#ff0}}", class_name);
    assert_eq!(style, expected);
}

#[test]
fn test_19() {
    let (class_name, style) = style_str! {
        div[class$=test] {
            background-color: yellow;
        }
    };
    let expected = format!("div[class$=test].{}{{background-color:#ff0}}", class_name);
    assert_eq!(style, expected);
}

#[test]
fn test_20() {
    let (class_name, style) = style_str! {
        div [class$=test] {
            background-color: yellow;
        }
    };
    let expected = format!("div.{} [class$=test].{}{{background-color:#ff0}}", class_name, class_name);
    assert_eq!(style, expected);
}

#[test]
fn test_21() {
    let (class_name, style) = style_str! {
        div[class*="test"] {
            background-color: yellow;
        }
    };
    let expected = format!("div[class*=test].{}{{background-color:#ff0}}", class_name);
    assert_eq!(style, expected);
}

#[test]
fn test_22() {
    let (class_name, style) = style_str! {
        .one:hover{
            background-color: green;
        }
    };
    let expected = format!(".one.{}:hover{{background-color:green}}", class_name);
    assert_eq!(style, expected);
}

#[test]
fn test_23() {
    // ++ Difference from [stylers](https://github.com/abishekatp/stylers)
    // non standard raw_str will not be supported...
    let (class_name, style) = style_str! {
        p::before {
            content: "Read this: ";
        }
    };
    let expected = format!("p.{}:before{{content:\"Read this: \"}}", class_name);
    assert_eq!(style, expected);
}

#[test]
fn test_24() {
    let (class_name, style) = style_str! {
        div:nth-child(2){
            background-color: green;
        }
    };
    let expected = format!("div.{}:nth-child(2){{background-color:green}}", class_name);
    assert_eq!(style, expected);
}

#[test]
fn test_25() {
    let (class_name, style) = style_str! {
        p:lang(it){
            background: yellow;
        }
    };
    let expected = format!("p.{}:lang(it){{background:#ff0}}", class_name);
    assert_eq!(style, expected);
}

#[test]
fn test_26() {
    // Difference from [stylers](https://github.com/abishekatp/stylers)
    // Namespaces do not have custom class appended.
    let (class_name, style) = style_str! {
        svg|a {
        }
    };
    let expected = format!("svg|a.{}{{}}", class_name);
    assert_eq!(style, expected);
}

#[test]
#[ignore]
// TODO: Waiting on outcome of this discussion:
//       https://github.com/parcel-bundler/lightningcss/issues/310
//       If minify removes due to UTF-8 being a default, then change charset to be tested.
//       Current. ParserOptions may have to change.
fn test_27() {
    let (_, style) = style_str! {
        @charset "ASCII";
    };
    let expected = r#"@charset "UTF-8";"#;
    assert_eq!(style, expected);
}

#[test]
#[ignore]
// TODO: Waiting on outcome of this discussion:
//       https://github.com/parcel-bundler/lightningcss/issues/555
//       ParserOptions may have to change.
fn test_28() {
    let (_, style) = style_str! {
        @import url("landscape.css") screen and (orientation: landscape);
    };
    let expected = r#"@import url("landscape.css") screen and (orientation: landscape);"#;
    assert_eq!(style, expected);
}

#[test]
#[ignore]
// TODO: Waiting on outcome of this discussion:
//       https://github.com/parcel-bundler/lightningcss/issues/555
//       ParserOptions may have to change.
fn test_29() {
    let (_, style) = style_str! {
        @namespace svg url("http://www.w3.org/2000/svg");
    };
    let expected = r#"@namespace svg url("http://www.w3.org/2000/svg");"#;
    assert_eq!(style, expected); 
}

#[test]
fn test_30() {
    let (class_name, style) = style_str! {
        @supports (display: flex) {
            @media screen and (min-width: 900px) {
                article {
                    display: flex;
                }
            }
        }
    };
    let expected = format!("@supports (display:flex){{@media screen and (width>=900px){{article.{}{{display:flex}}}}}}", class_name);
    assert_eq!(style, expected);
}

#[test]
fn test_31() {
    let (_, style) = style_str! {
        @document url("https://www.example.com/")
        {
            h1 {
                color: green;
            }
        }
    };
    let expected = "@document url(https://www.example.com/){h1 { color: green; }}";
    assert_eq!(style, expected);
}

#[test]
fn test_32() {
    let (_, style) = style_str! {
        @page {
            size: A4;
            margin: 10%;

            @top-left-corner {
            content: "Page " counter(page);
            }
        }
    };
    let expected = r#"@page{size:A4;margin:10%;@top-left-corner{content:"Page " counter(page)}}"#;
    assert_eq!(style, expected);
}

#[test]
fn test_33() {
    let (_, style) = style_str! {
        @font-face {
            font-family: "Trickster";
            src: local("Trickster"),
            url("trickster-COLRv1.otf") format("opentype") tech(color-COLRv1), url("trickster-outline.otf")
                format("opentype"), url("trickster-outline.woff") format("woff");
        }
    };
    let expected = r#"@font-face{font-family:Trickster;src:local(Trickster),url(trickster-COLRv1.otf)format("opentype")tech(color-colrv1),url(trickster-outline.otf)format("opentype"),url(trickster-outline.woff)format("woff")}"#;
    assert_eq!(style, expected);
}

#[test]
fn test_34() {
    let (_, style) = style_str! {
        @keyframes spin1 {
            to {
                -webkit-transform: rotate(360deg);
            }
        }
    };
    let expected = "@keyframes spin1{to{-webkit-transform:rotate(360deg)}}";
    assert_eq!(style, expected);
}

#[test]
fn test_35() {
    let (_, style) = style_str! {
        @-webkit-keyframes spin2 {
            to {
                -webkit-transform: rotate(360deg);
            }
        }
    };
    let expected = "@-webkit-keyframes spin2{to{-webkit-transform:rotate(360deg)}}";
    assert_eq!(style, expected);
}

#[test]
fn test_36() {
    let (_, style) = style_str! {
        @counter-style thumbs {
            system: cyclic;
            symbols: "\01F44D";
            suffix: " ";
        }
    };
    let expected = r#"@counter-style thumbs{system:cyclic;symbols:"👍";suffix:" "}"#;
    assert_eq!(style, expected);
}

#[test]
fn test_37() {
    let (_, style) = style_str! {
        @font-feature-values Font One {
            @styleset {
                nice-style: 12;
            }
        }
    };
    // Experimental feature; not minimised by lightningcss.
    let expected = r#"@font-feature-values Font One{@styleset { nice-style: 12; }}"#;
    assert_eq!(style, expected);
}

#[test]
fn test_38() {
    let (_, style) = style_str! {
        @property --property-name {
            syntax: "<color>";
            inherits: false;
            initial-value: #c0ffee;
        }
    };
    let expected = r#"@property --property-name{syntax:"<color>";inherits:false;initial-value:#c0ffee}"#;
    assert_eq!(style, expected);
}

#[test]
#[ignore]
// TODO: This test fails. In order to pass the rust compilation a single (required) '\' needs to be
//       escaped. This results in the raw escape sequence ('\\') in the resultant css. Note: stylers
//       resolves this by introducing a raw_str function, which is not supported here.
fn test_39() {
    let (class_name, style) = style_str! {
        @layer framework {
            @layer layout {
                p {
                    margin-block: 1rem;
                    font: "0.9em/1.2" Arial, Helvetica, sans-serif;
                    content: "\\hello";
                }
            }
        }
    };
    let expected = format!(r#"@layer framework{{@layer layout{{p.{}{{margin-block:1rem;font:"0.9em/1.2" Arial,Helvetica,sans-serif;content:"\hello"}}}}}}"#, class_name);
    assert_eq!(style, expected);
}

#[test]
fn test_40() {
    let (_, style) = style_str! {
        @layer theme, layout, utilities;
    };
    let expected = r#"@layer theme,layout,utilities;"#;
    assert_eq!(style, expected);
}

#[test]
fn test_41() {
    let (class_name, style) = style_str! {
        :not(body) {
            background: #ff0000;
        }
    };
    let expected = format!(".{}:not(body){{background:red}}", class_name);
    assert_eq!(style, expected);
}

#[test]
fn test_42() {
    let (class_name, style) = style_str! {
        :root {
            --blue: #1e90ff;
        }

        body { background-color: var(--blue); }
    };
    let expected = format!(":root{{--blue:#1e90ff}}body.{}{{background-color:var(--blue)}}", class_name);
    assert_eq!(style, expected);
}

#[test]
fn test_43() {
    let (class_name, style) = style_str! {
        #container {
            --first-color: #290;
        }
        #thirdParagraph {
            background-color: var(--first-color);
            color: var(--second-color);
        }
    };
    let expected = format!("#container.{}{{--first-color:#290}}#thirdParagraph.{}{{background-color:var(--first-color);color:var(--second-color)}}", class_name, class_name);
    assert_eq!(style, expected);
}

#[test]
fn test_44() {
    let (class_name, style) = style_str! {
        table th,
        table td {
            color: red;
        }
    };
    let expected = format!("table.{} th.{},table.{} td.{}{{color:red}}", class_name, class_name, class_name, class_name);
    assert_eq!(style, expected);
}

#[test]
fn test_45() {
    let (class_name, style) = style_str! {
        div :deep(h3) {
            color: orange;
        }
    };
    let expected = format!("div.{} :deep(h3){{color:orange}}", class_name);
    assert_eq!(style, expected);
}

#[test]
fn test_46() {
    let (_, style) = style_str! {
        :deep(h3 div) {
            color: orange;
        }
    };
    let expected = ":deep(h3 div){color:orange}";
    assert_eq!(style, expected);
}

#[test]
fn test_47() {
    let (class_name, style) = style_str! {
        div> :deep(h3) {
            color: orange;
        }
    };
    let expected = format!("div.{}>:deep(h3){{color:orange}}", class_name);
    assert_eq!(style, expected);
}

#[test]
fn test_48() {
    let (_, style) = style_str! {
        :deep([data-custom]) {
            color: orange;
        }
    };
    let expected = ":deep([data-custom]){color:orange}";
    assert_eq!(style, expected);
}

#[test]
fn test_49() {
    let (class_name, style) = style_str! {
        .nested> :deep([data-custom]) {
            color: orange;
        }
    };
    let expected = format!(".nested.{}>:deep([data-custom]){{color:orange}}", class_name);
    assert_eq!(style, expected);
}

#[test]
fn test_50() {
    let (class_name, style) = style_str! {
        @supports (display: flex) {
            .flex-container > * {
                text-shadow: 0 0 2px blue;
                float: none;
            }

            .flex-container {
                display: flex;
            }
        }
    };
    let expected = format!("@supports (display:flex){{.flex-container.{}>.{}{{text-shadow:0 0 2px #00f;float:none}}.flex-container.{}{{display:flex}}}}", class_name, class_name, class_name);
    assert_eq!(style, expected);
}

#[test]
fn test_51() {
    let (class_name, style) = style_str! {
        :deep(.rollUp) .unlockSplash {
            max-height: 0;
        }
    };
    let expected = format!(":deep(.rollUp) .unlockSplash.{}{{max-height:0}}", class_name);
    assert_eq!(style, expected);
}

#[test]
fn test_52() {
    let (class_name, style) = style_str! {
        .unitToggle :deep(.onDisplay),
        .unitToggle :deep(.offDisplay) {
            color: black;
        }
    };
    let expected = format!(".unitToggle.{} :deep(.onDisplay),.unitToggle.{} :deep(.offDisplay){{color:#000}}", class_name, class_name);
    assert_eq!(style, expected);
}

#[test]
fn test_53() {
    let (class_name, style) = style_str! {
         .wingman :deep(svg[role=graphics-symbol]) {
           width: 100%;
        }
    };
    let expected = format!(".wingman.{} :deep(svg[role=graphics-symbol]){{width:100%}}", class_name);
    assert_eq!(style, expected);
}

#[test]
fn test_54() {
    let (class_name, style) = style_str! {
        .errorSign {
            transform-box: fill-box;
            transform-origin: center;
            scrollbar-width: 1px;
            scrollbar-color: red;
        }
    };
    let expected = format!(".errorSign.{}{{transform-box:fill-box;transform-origin:50%;scrollbar-width:1px;scrollbar-color:red}}", class_name);
    assert_eq!(style, expected);
}
