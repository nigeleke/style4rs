// These samples follow the style4rs macro tests in the main style4rs crate.
// These are picked up during the testing of the Style4rsBuilder. Thos enables
// gathering of coverage stats, which isn't possible during the compile-time
// tests in style4rs.
//
// Note: The module isn't actually compiled, but it _is_ processed
// by Style4rsBuilder;:build().
//
fn sample_test_1() {
    let (_, _) = style_str! {.two{
            /* This comment should be ignored */
            color: yellow;
        }
    };
}

fn sample_test_2() {
    let (_, _) = style_str! {
        .two.one  {
            color: yellow;
        }
    };
}

fn sample_test_3() {
    let (_, _) = style_str! {
        .two .one{
            color: yellow;
        }
    };
}

fn sample_test_4() {
    let (_, _) = style_str! {
        #firstname{
            background-color: yellow;
        }
    };
}

fn sample_test_5() {
    let (_, _) = style_str! {
        *{
            background-color: yellow;
        }
    };
}

fn sample_test_6() {
    let (_, _) = style_str! {
        div{
            border: 1px solid black;
            margin: 25px 50px 75px 100px;
            background-color: lightblue;
        }
    };
}

fn sample_test_7() {
    let (_, _) = style_str! {
        div .one p{
            color: blue;
        }
    };
}

fn sample_test_8() {
    let (_, _) = style_str! {
        div.one p div{
            color: blue;
        }
    };
}

fn sample_test_9() {
    let (_, _) = style_str! {
        div #two{
            color: blue;
        }
    };
}

fn sample_test_10() {
    let (_, _) = style_str! {
        h2 , a{
            color: purple;
        }
    };
}

fn sample_test_11() {
    let (_, _) = style_str! {
        div > p{
            background-color: yellow;
        }
    };
}

fn sample_test_12() {
    let (_, _) = style_str! {
        div + p {
            background-color: yellow;
        }
    };
}

fn sample_test_13() {
    let (_, _) = style_str! {
        p ~ ul {
            background: #ff0000;
        }
    };
}

fn sample_test_14() {
    let (_, _) = style_str! {
        a[target] {
            background-color: yellow;
        }
    };
}

fn sample_test_15() {
    let (_, _) = style_str! {
        a[title="I am ,testing"] {
            background-color: yellow;
        }
    };
}

fn sample_test_16() {
    let (_, _) = style_str! {
        [title~=flower] {
            background-color: yellow;
        }
    };
}

fn sample_test_17() {
    let (_, _) = style_str! {
        [lang|=en] {
            background-color: yellow;
        }
    };
}

fn sample_test_18() {
    let (_, _) = style_str! {
        div[class^="test"] {
            background-color: yellow;
        }
    };
}

fn sample_test_19() {
    let (_, _) = style_str! {
        div[class$=test] {
            background-color: yellow;
        }
    };
}

fn sample_test_20() {
    let (_, _) = style_str! {
        div [class$=test] {
            background-color: yellow;
        }
    };
}

fn sample_test_21() {
    let(_, _) = style_str! {
        div[class*="test"] {
            background-color: yellow;
        }
    };
}

fn sample_test_22() {
    let (_, _) = style_str! {
        .one:hover{
            background-color: green;
        }
    };
}

fn sample_test_23() {
    let (_, _) = style_str! {
        p::before {
            content: "Read this: ";
        }
    };
}

fn sample_test_24() {
    let (_, _) = style_str! {
        div:nth-child(2){
            background-color: green;
        }
    };
}

fn sample_test_25() {
    let (_, _) = style_str! {
        p:lang(it){
            background: yellow;
        }
    };
    let expected = format!("p.{}:lang(it){{background:#ff0}}", class_name);
    assert_eq!(style, expected);
}

fn sample_test_26() {
    let (_, _) = style_str! {
        svg|a {
        }
    };
    let expected = format!("svg|a.{}{{}}", class_name);
    assert_eq!(style, expected);
}

fn sample_test_27() {
    let (_, _) = style_str! {
        @charset "ASCII";
    };
}

fn sample_test_28() {
    let (_, _) = style_str! {
        @import url("landscape.css") screen and (orientation: landscape);
    };
}

fn sample_test_29() {
    let (_, _) = style_str! {
        @namespace svg url("http://www.w3.org/2000/svg");
    };
}

fn sample_test_30() {
    let (_, _) = style_str! {
        @supports (display: flex) {
            @media screen and (min-width: 900px) {
                article {
                    display: flex;
                }
            }
        }
    };
}

fn sample_test_31() {
    let (_, _) = style_str! {
        @document url("https://www.example.com/")
        {
            h1 {
                color: green;
            }
        }
    };
}

fn sample_test_32() {
    let (_, _) = style_str! {
        @page {
            size: A4;
            margin: 10%;

            @top-left-corner {
            content: "Page " counter(page);
            }
        }
    };
}

fn sample_test_33() {
    let (_, _) = style_str! {
        @font-face {
            font-family: "Trickster";
            src: local("Trickster"),
            url("trickster-COLRv1.otf") format("opentype") tech(color-COLRv1), url("trickster-outline.otf")
                format("opentype"), url("trickster-outline.woff") format("woff");
        }
    };
}

fn sample_test_34() {
    let (_, _) = style_str! {
        @keyframes spin1 {
            to {
                -webkit-transform: rotate(360deg);
            }
        }
    };
}

fn sample_test_35() {
    let (_, _) = style_str! {
        @-webkit-keyframes spin2 {
            to {
                -webkit-transform: rotate(360deg);
            }
        }
    };
}

fn sample_test_36() {
    let (_, _) = style_str! {
        @counter-style thumbs {
            system: cyclic;
            symbols: "\01F44D";
            suffix: " ";
        }
    };
}

fn sample_test_37() {
    let (_, _) = style_str! {
        @font-feature-values Font One {
            @styleset {
                nice-style: 12;
            }
        }
    };
}

fn sample_test_38() {
    let (_, _) = style_str! {
        @property --property-name {
            syntax: "<color>";
            inherits: false;
            initial-value: #c0ffee;
        }
    };
}

fn sample_test_39() {
    let (_, _) = style_str! {
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
}

fn sample_test_40() {
    let (_, _) = style_str! {
        @layer theme, layout, utilities;
    };
}

fn sample_test_41() {
    let (_, _) = style_str! {
        :not(body) {
            background: #ff0000;
        }
    };
}

fn sample_test_42() {
    let (_, _) = style_str! {
        :root {
            --blue: #1e90ff;
        }

        body { background-color: var(--blue); }
    };
}

fn sample_test_43() {
    let (_, _) = style_str! {
        #container {
            --first-color: #290;
        }
        #thirdParagraph {
            background-color: var(--first-color);
            color: var(--second-color);
        }
    };
}

fn sample_test_44() {
    let (_, _) = style_str! {
        table th,
        table td {
            color: red;
        }
    };
}

fn sample_test_45() {
    let (_, _) = style_str! {
        div :deep(h3) {
            color: orange;
        }
    };
}

fn sample_test_46() {
    let (_, _) = style_str! {
        :deep(h3 div) {
            color: orange;
        }
    };
}

fn sample_test_47() {
    let (_, _) = style_str! {
        div> :deep(h3) {
            color: orange;
        }
    };
}

fn sample_test_48() {
    let (_, _) = style_str! {
        :deep([data-custom]) {
            color: orange;
        }
    };
}

fn sample_test_49() {
    let (_, _) = style_str! {
        .nested> :deep([data-custom]) {
            color: orange;
        }
    };
}

fn sample_test_50() {
    let (_, _) = style_str! {
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
}

fn sample_test_51() {
    let (_, _) = style_str! {
        :deep(.rollUp) .unlockSplash {
            max-height: 0;
        }
    };
}

fn sample_test_52() {
    let (_, _) = style_str! {
        .unitToggle :deep(.onDisplay),
        .unitToggle :deep(.offDisplay) {
            color: black;
        }
    };
}

fn sample_test_53() {
    let (_, _) = style_str! {
         .wingman :deep(svg[role=graphics-symbol]) {
           width: 100%;
        }
    };
}

fn sample_test_54() {
    let (_, _) = style_str! {
        .errorSign {
            transform-box: fill-box;
            transform-origin: center;
            scrollbar-width: 1px;
            scrollbar-color: red;
        }
    };
}
