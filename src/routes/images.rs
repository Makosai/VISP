use dioxus::prelude::*;

#[component]
pub (in crate::routes) fn Images() -> Element {
    rsx! {
        h1 {
            class: "font-black text-2xl",
            "Images"
        }
    }
}