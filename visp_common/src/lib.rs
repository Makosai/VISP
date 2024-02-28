//! VISP - main.rs
//! Authors: Kristopher Ali (Makosai)
//!
//! This is the entrypoint for VISP's common code.

#![allow(non_snake_case, unused)]

use dioxus::html::style;
use dioxus::prelude::*;
use dioxus_router::prelude::*;


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
        div {
            class: "bg-black font-black w-full h-full",
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
