use style4rs::style;

pub fn the_class_name() -> String {
    let class_name = style!{
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