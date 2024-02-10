#![crate_type = "lib"]
#![crate_name = "visp_common"]

#![allow(non_snake_case, unused)]

use dioxus_router::prelude::*;
use dioxus::prelude::*;

#[cfg(feature = "web")]
use dioxus_fullstack::prelude::*;

// https://dioxuslabs.com/learn/0.4/router/example/full-code
#[derive(Routable, PartialEq, Debug, Clone)]
enum Route {
    #[layout(NavBar)]
    #[route("/")]
    Home {},
    #[route("/other")]
    Other {},
    #[end_layout]
    //  if the current location doesn't match any of the above routes, render the NotFound component
    #[route("/:..route")]
    NotFound { route: Vec<String> },
}

#[component]
fn NavBar(cx: Scope) -> Element {
    render! {
        nav {
            ul {
                li {
                    Link { to: Route::Home {}, "Home" }
                }
                li {
                    Link { to: Route::Other {}, "Other" }
                }
            }
        }
        Outlet::<Route> {}
    }
}


#[component]
fn Home(cx: Scope) -> Element {
    let mut count = use_state(cx, || 0);

    render! {
        h1 { "Count me: {count}" }
        button { onclick: move |_| count += 1, "Increase" }
        button { onclick: move |_| count -= 1, "Decrease" }
    }
}

#[component]
fn Other(cx: Scope) -> Element {
    render! {
        h1 { "This is a temporary second page." }
    }
}

#[component]
fn NotFound(cx: Scope, route: Vec<String>) -> Element {
    render! {
        h1 { "Page not found" }
        p { "We are terribly sorry, but the page you requested doesn't exist." }
        pre { color: "red", "log:\nattemped to navigate to: {route:?}" }
    }
}

pub fn app(cx: Scope) -> Element {
    render! {
        Router::<Route> { }
    }
}

#[cfg(all(feature = "desktop", not(feature = "web")))]
pub fn start_app(app: fn(Scope) -> Element) {
    dioxus_desktop::launch(app);
}

#[cfg(all(feature = "web", not(feature = "desktop")))]
pub fn start_app(app: fn(Scope) -> Element) {
    LaunchBuilder::new(app).launch()
}
