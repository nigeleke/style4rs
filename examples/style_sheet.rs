use leptos::*;
use style4rs::style_sheet;

#[component]
pub fn MyStyleSheetComponent() -> impl IntoView {

    let class_name = style_sheet!("resources/style_sheet.css");

    view! {
        class = class_name,
        <div>
            <p>"One"</p>
            <p>"Two"</p>
            <p>"Three"</p>
        </div>
    }
}
