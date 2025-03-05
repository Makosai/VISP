//! web.rs
//! Authors: Kristopher Ali (Makosai)
//!
//! This is the entrypoint for VISP's web.

use dioxus::prelude::*;

pub(crate) fn start_app(app: fn() -> Element) {
    launch(app);
}