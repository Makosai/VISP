use dioxus::prelude::*;

#[component]
pub (in crate::routes) fn Home(cx: Scope) -> Element {
    let mut count = use_state(cx, || 0);

    render! {
        div {
            class: "bg-[--boring-gray] font-black w-full h-full",
            video {
                width: "auto", height: "auto", autoplay: true, muted: true, src: ""
            }
            p { "Hello" }
            h1 { class: "text-red font-black", "Count me 2: {count}" }
            button { onclick: move |_| count += 1, "Increase" }
            button { onclick: move |_| count -= 1, "Decrease" }
        }
    }
}