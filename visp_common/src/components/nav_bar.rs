use dioxus::prelude::*;
use dioxus_router::prelude::*;

use crate::routes::Route;

#[component]
pub(crate) fn NavBar(cx: Scope) -> Element {
    render! {
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
                }
            }
            Outlet::<Route> {}
        }
    }
}