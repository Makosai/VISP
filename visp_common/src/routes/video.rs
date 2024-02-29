use dioxus::prelude::*;

#[component]
pub(in crate::routes) fn Video(cx: Scope) -> Element {
    render! {
        div {
            class: "page",
            h1 {
                class: "font-black text-2xl",
                "Video"
            }
            div {
                class: "flex slot-70 border-black border-2",
                div {
                    class: "slot-30",
                    p { "30% | Info Windows" }
                }
                div {
                    class: "slot-70",
                    p { "70% | Video & Effects Previews" }
                }
            }
            div {
                class: "flex flex-col slot-30 border-black border-2",
                div {
                    class: "slot-10",
                    p { "10% | Toolbar" }
                }
                div {
                    class: "slot-90",
                    p { "90% | Timeline" }
                }
            }
        }
    }
}