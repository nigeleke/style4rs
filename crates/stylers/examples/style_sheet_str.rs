use leptos::*;
use style4rs::style_sheet_str;

#[component]
pub fn MyStyleSheetComponent() -> impl IntoView {

    let (class_name, style) = style_sheet_str!("examples/resources/style_sheet_str.css");

    view! {
        class = class_name,
        <style>{style}</style>
        <div>
            <p>"One"</p>
            <p>"Two"</p>
            <p>"Three"</p>
        </div>
    }
}

fn main() {}