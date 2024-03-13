use dioxus::prelude::*;
use dioxus_router::prelude::*;

use crate::routes::Route;

#[component]
pub(crate) fn NavBar() -> Element {
    rsx! {
        div {
            class: "page",
            nav {
                ul {
                    class: "flex gap-2",
                    li {
                        Link { to: Route::Home {}, "Home" }
                    }
                    li {
                        Link { to: Route::Video {}, "Video" }
                    }
                    li {
                        Link { to: Route::Images {}, "Images" }
                    }
                    li {
                        Link { to: Route::Sound {}, "Sound" }
                    }
                    li {
                        Link { to: Route::Post {}, "Post" }
                    }
                }
            }
            Outlet::<Route> {}
        }
    }
}