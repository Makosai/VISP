use dioxus::prelude::*;

#[component]
pub(in crate::routes) fn Home() -> Element {
    let username = use_signal(|| "Kristopher");

    rsx! {
        div {
            class: "w-full h-full",
            div {
                class: "flex gap-4 items-center mt-4",
                div {
                    p {
                        class: "text-3xl",
                        "Hello "
                        strong {
                            "{username}! 🎉"
                        }
                    }
                }
                // Search box with rounded border and search icon embedded in it
                div {
                    class: "flex items-center gap-2 relative inline-block",
                    input {
                        class: "rounded-3xl py-4 pl-14 pr-4 absolute bg-[--black] placeholder-[--gray]",
                        r#type: "text",
                        placeholder: "Search...  CTRL+F",
                    }
                    i {
                        class: "fa-solid fa-magnifying-glass z-10 pl-6 text-[--gray]"
                    }
                }
            }
        }
    }
}
