use style4rs::*;

#[test]
fn it_can_be_invoked() {
    let (class_name, _) = style_str! {
        div {
            color: white;
            background-color: black;
        }
    };
    assert!(class_name.starts_with("rcn-"));
}

fn the_class_name() -> String {
    let (class_name, _) = style_str!{
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
    class_name.to_string()
}

#[test]
fn class_name_is_deterministic() {
    let (class_name, _) = style_str! {
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
