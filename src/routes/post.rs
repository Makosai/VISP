use dioxus::prelude::*;

#[component]
pub (in crate::routes) fn Post() -> Element {
    rsx! {
        h1 {
            class: "font-black text-2xl",
            "Post"
        }
    }
}