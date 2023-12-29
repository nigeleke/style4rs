use leptos::*;
use style4rs::style_str;

#[component]
pub fn MyStyleStrComponent() -> impl IntoView {

    let (class_name, style) = style_str!(
        div {
            display: flex;
            flex-direction: row;
            justify-content: space-between;
        }
        div:last-child {
            align-self: flex-end;
        }
    );

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