//! VISP - main.rs
//! Authors: Kristopher Ali (Makosai)
//!
//! This is the entrypoint for VISP's desktop.

#![allow(non_snake_case)]
#![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]

// VISP
use visp_common;

// Dioxus
use dioxus_desktop::{Config, LogicalSize, WindowBuilder};

use std::fs::{self};
use dioxus_desktop::tao::dpi::{PhysicalPosition};
use platform_dirs::AppDirs;

/// Generates some html headers.
///
/// The generated output is based on the deployment type.
/// It's currently based on whether you're in release mode.
/// This is needed because the public path changes when deployed.
fn get_head() -> String {
    return if cfg!(debug_assertions) {
        format!(r#"<link rel="stylesheet" type="text/css" href="{}tailwind.css">"#, "../visp_common/public/assets/css/", )
    } else {
        format!(r#"<link rel="stylesheet" type="text/css" href="{}tailwind.css">"#, "/assets/css/")
    };
}

fn main() {
    let app_dirs = AppDirs::new(Some("Quaint Studios/VISP"), true).unwrap();

    fs::create_dir_all(&app_dirs.data_dir).unwrap();
    println!("Hi! I'm Paul.");

    dioxus_desktop::launch_cfg(visp_common::app, Config::default()
        .with_data_directory(&app_dirs.data_dir)
        .with_resource_directory(app_dirs.data_dir)
        .with_window(
            WindowBuilder::new()
                .with_title("VISP - Video. Images. Sound. Post.")
                .with_inner_size(LogicalSize::new(1280, 720))
                .with_position(PhysicalPosition::new(100, 100))
        )
        .with_custom_head(get_head().into()))
}