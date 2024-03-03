//! VISP - main.rs
//! Authors: Kristopher Ali (Makosai)
//!
//! This is the entrypoint for VISP's common code.

#![allow(non_snake_case, unused)]
#![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]

// Modules
mod routes;
mod components;

#[cfg_attr(target_family = "windows", path = "desktop.rs")]
#[cfg_attr(target_family = "wasm", path = "web.rs")]
mod core;

// Dioxus
use dioxus::prelude::*;
use dioxus_router::prelude::*;

// Imports
use manganis::mg;
use routes::Route;

const _STYLE: &str = mg!(file("public/assets/css/tailwind.css"));

fn main() {
    core::start_app(app);
}

fn app() -> Element {
    rsx!(
        Router::<Route> { }
    )
}