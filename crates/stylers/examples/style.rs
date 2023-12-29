use leptos::*;
use style4rs::style;

#[component]
pub fn MyStyleComponent() -> impl IntoView {

    let class_name = style!(
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
        <div>
            <p>"One"</p>
            <p>"Two"</p>
            <p>"Three"</p>
        </div>
    }
}

fn main() {}