use dioxus::prelude::*;

#[component]
pub (in crate::routes) fn Video(cx: Scope) -> Element {
    render! {
        h1 { "This is a temporary second page." }
    }
}