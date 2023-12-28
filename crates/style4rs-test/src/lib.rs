use style4rs::*;

pub fn style_class_name() -> String {
    let class_name = style!{
        #one{
            color: blue;
        }
    };
    class_name.to_string()
}

pub fn style_str_class_name() -> String {
    let (class_name, _) = style_str!{
        #one{
            color: red;
        }
    };
    class_name.to_string()
}