//! VISP - main.rs
//! Authors: Kristopher Ali (Makosai)
//!
//! This is the entrypoint for VISP's common code.

#![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]

// Modules
mod components;
mod routes;

#[path = "modules/video/mod.rs"]
mod video;

#[cfg_attr(target_family = "windows", path = "desktop.rs")]
#[cfg_attr(target_family = "unix", path = "desktop.rs")]
#[cfg_attr(target_family = "wasm", path = "web.rs")]
mod core;

// Dioxus
use dioxus::prelude::*;

// Imports
use routes::Route;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/css/main.css");
const FONTAWESOME_CSS: Asset = asset!("/assets/fontawesome/css/fontawesome.min.css");
const FONTAWESOME_SOLID_CSS: Asset = asset!("/assets/fontawesome/css/solid.min.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    core::start_app(app);
}

fn app() -> Element {
    rsx!(
        // Global app resources
        document::Link { rel: "icon", href: FAVICON }

        // document::Link { rel: "preload", href: MAIN_CSS, as: "style" }
        // document::Link { rel: "preload", href: FONTAWESOME_CSS, as: "style" }
        // document::Link { rel: "preload", href: FONTAWESOME_SOLID_CSS, as: "style" }
        // document::Link { rel: "preload", href: TAILWIND_CSS, as: "style" }

        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: FONTAWESOME_CSS }
        document::Link { rel: "stylesheet", href: FONTAWESOME_SOLID_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }

        Router::<Route> {}
    )
}
