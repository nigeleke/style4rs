use style4rs_macros::*;

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
        #two{
            color: red;
        }
    };
    class_name.to_string()
}

pub fn style_sheet_class_name() -> String {
    let class_name = style_sheet!("resources/style_sheet.css");
    class_name.to_string()
}

pub fn style_sheet_str_class_name() -> String {
    let (class_name, _) = style_sheet_str!("resources/style_sheet_str.css");
    class_name.to_string()
}
