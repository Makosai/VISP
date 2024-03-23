//! VISP - main.rs
//! Authors: Kristopher Ali (Makosai)
//!
//! This is the entrypoint for VISP's common code.

#![allow(non_snake_case)]
#![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]

// Modules
mod routes;
mod components;

#[path = "modules/video/mod.rs"]
mod video;

#[cfg_attr(target_family = "windows", path = "desktop.rs")]
#[cfg_attr(target_family = "wasm", path = "web.rs")]
mod core;

// Dioxus
use dioxus::prelude::*;
use dioxus_router::prelude::*;

// Imports
use routes::Route;

fn main() {
    core::start_app(app);
}

fn app() -> Element {
    rsx!(
        style {
            {include_str!("../public/assets/css/tailwind.css")}
            {include_str!("../public/assets/fontawesome/css/fontawesome.min.css")}
            {include_str!("../public/assets/fontawesome/css/solid.min.css")}
        }
        Router::<Route> { }
    )
}