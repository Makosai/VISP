use dioxus::prelude::*;

#[component]
pub (in crate::routes) fn Home() -> Element {
    let mut count = use_signal(|| 0);

    rsx! {
        h1 {
            class: "font-black text-2xl",
            "Home"
        }
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
