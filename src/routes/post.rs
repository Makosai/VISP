use dioxus::prelude::*;

#[component]
pub(in crate::routes) fn Post() -> Element {
    rsx! {
        document::Title { "VISP - Video. Image. Sound. Post." }
        h1 { class: "font-black text-2xl", "Post" }
    }
}
