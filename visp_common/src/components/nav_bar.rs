use dioxus::prelude::*;
use dioxus_router::prelude::*;

use crate::routes::Route;

#[component]
pub fn NavBar(cx: Scope) -> Element {
    render! {
        nav {
            ul {
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