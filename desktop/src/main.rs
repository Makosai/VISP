#![allow(non_snake_case)]
#![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]

// VISP
use visp_common;

// Dioxus
use dioxus_desktop::{Config, LogicalSize, WindowBuilder};

use std::fs::{self};
use dioxus_desktop::tao::dpi::{LogicalPosition, PhysicalPosition};
use platform_dirs::AppDirs;

fn main() {
    let app_dirs = AppDirs::new(Some("Quaint Studios/VISP"), true).unwrap();

    fs::create_dir_all(&app_dirs.data_dir).unwrap();
    println!("Hi");

    dioxus_desktop::launch_cfg(visp_common::app, Config::default()
        .with_data_directory(&app_dirs.data_dir)
        .with_resource_directory(app_dirs.data_dir)
        .with_window(
            WindowBuilder::new()
                .with_title("VISP - Home")
                .with_inner_size(LogicalSize::new(1280, 720))
                .with_position(PhysicalPosition::new(100, 100))
        )
        .with_custom_head(r#"<link rel="stylesheet" type="text/css" href="/assets/css/tailwind.css">"#.into()))
}