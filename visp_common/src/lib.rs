//! VISP - main.rs
//! Authors: Kristopher Ali (Makosai)
//!
//! This is the entrypoint for VISP's common code.

#![allow(non_snake_case, unused)]

use dioxus::prelude::*;
use dioxus_router::prelude::*;

mod routes;
mod components;

use routes::Route;

pub fn app(cx: Scope) -> Element {
    render! {
        Router::<Route> { }
    }
}
