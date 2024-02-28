//! VISP - main.rs
//! Authors: Kristopher Ali (Makosai)
//!
//! This is the entrypoint for VISP's web.

#![allow(non_snake_case)]

// VISP
use visp_common;

// Dioxus
// use dioxus::prelude::*;

fn main() {
    // launch the web app
    dioxus_web::launch(visp_common::app);
}