use dioxus::prelude::*;

use crate::routes::Route;

// Profile component
#[component]
pub(crate) fn Profile() -> Element {
    rsx! {
        div {
            // A profile info section
            class: "flex flex-col items-center gap-2",
            img {
                class: "h-16 w-16 rounded-full",
                width: "64",
                height: "64",
                src: "https://avatars.githubusercontent.com/u/25123509?v=4",
                alt: "Profile Picture"
            }
            p {
                class: "font-bold",
                "Kristopher Ali"
            }
            p {
                class: "font-thin text-sm",
                "Software Developer"
            }
        }
    }
}

//Tagline component
#[component]
pub(crate) fn Headline() -> Element {
    rsx! {
        div {
            p {
                class: "font-black",
                "VISP"
            }
            p {
                class: "font-thin text-sm text-[--gray]",
                "Video. Image. Sound. Post."
            }
        }
    }
}

#[component]
pub(crate) fn NavBar() -> Element {
    // keep track of which button was pressed last
    let mut selected = use_signal(|| "Home");

    rsx! {
        div {
            div {
                class: "sidebar",
                div {
                    class: "font-bold text-2xl flex gap-2 justify-center items-center",
                    img {
                        class: "rounded-full",
                        width: "64",
                        height: "32",
                        src: "/assets/images/logo.webp",
                        alt: "VISP Logo"
                    }
                    Headline {}
                }
                nav {
                    ul {
                        class: "flex flex-col gap-2 justify-center",
                        li {
                            // class is set to "selected" if the button was pressed last
                            class: if selected == "Home" { "selected" } else { "" },
                            Link {
                                to: Route::Home {},
                                onclick: move |_| selected.set("Home"),
                                i { class: "fa-solid fa-house" } "Home"
                            }
                        }
                        li {
                            class: if selected == "Video" { "selected" } else { "" },
                            Link {
                                to: Route::Video {},
                                onclick: move |_| selected.set("Video"),
                                i { class: "fa-solid fa-video" } "Video"
                            }
                        }
                        li {
                            class: if selected == "Images" { "selected" } else { "" },
                            Link {
                                to: Route::Images {},
                                onclick: move |_| selected.set("Images"),
                                i { class: "fa-solid fa-image" } "Images"
                            }
                        }
                        li {
                            class: if selected == "Sound" { "selected" } else { "" },
                            Link {
                                to: Route::Sound {},
                                onclick: move |_| selected.set("Sound"),
                                i { class: "fa-solid fa-music" } "Sound"
                            }
                        }
                        li {
                            class: if selected == "Post" { "selected" } else { "" },
                            Link {
                                to: Route::Post {},
                                onclick: move |_| selected.set("Post"),
                                i { class: "fa-solid fa-comment" } "Post"
                            }
                        }
                    }
                }
            }
            div {
                class: "main",
                Outlet::<Route> {}
            }
        }
    }
}