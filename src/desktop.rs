//! desktop.rs
//! Authors: Kristopher Ali (Makosai)
//!
//! This is the entrypoint for VISP's desktop.

use std::fs::{self};
use platform_dirs::AppDirs;

// Dioxus
use dioxus::desktop::{Config, LogicalSize, WindowBuilder};
use dioxus::desktop::tao::dpi::PhysicalPosition;
use dioxus::dioxus_core::Element;

pub(crate) fn start_app(app: fn() -> Element) {
    let app_dirs = AppDirs::new(Some("Quaint Studios/VISP"), true).unwrap();

    fs::create_dir_all(&app_dirs.data_dir).unwrap();
    println!("Hi! I'm Paul.");

    let config = Config::default()
        .with_data_directory(&app_dirs.data_dir)
        .with_resource_directory(app_dirs.data_dir)
        .with_window(
            WindowBuilder::new()
                .with_title("VISP - Video. Image. Sound. Post.")
                .with_inner_size(LogicalSize::new(1280, 720))
                .with_position(PhysicalPosition::new(100, 100))
        );

    dioxus::prelude::LaunchBuilder::desktop().with_cfg(config).launch(app);
}