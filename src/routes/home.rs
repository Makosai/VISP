use dioxus::prelude::*;

#[component]
pub(in crate::routes) fn Home() -> Element {
    let username = use_signal(|| "User");

    rsx! {
        div { class: "w-full h-full",
            div { class: "flex gap-4 items-center mt-1.5",
                div {
                    p { class: "text-3xl",
                        "Hello "
                        strong { "{username}! 🎉" }
                    }
                }
                // Search box with rounded border and search icon embedded in it
                div { class: "flex items-center gap-2 relative inline-block",
                    input {
                        class: "relative rounded-3xl py-4 pl-14 pr-4 bg-v-black placeholder-v-gray",
                        r#type: "text",
                        placeholder: "Search...  CTRL+F",
                    }
                    i { class: "absolute left-0 top-1/2 -translate-y-1/2 fa-solid fa-magnifying-glass z-10 pl-6 text-v-gray" }
                }
            }
        }
    }
}
