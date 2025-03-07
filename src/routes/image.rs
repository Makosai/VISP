use dioxus::prelude::*;

#[component]
pub(in crate::routes) fn Image() -> Element {
    rsx! {
        document::Title { "VISP - Video. Image. Sound. Post." }
        h1 { class: "font-black text-2xl", "Image" }
    }
}
