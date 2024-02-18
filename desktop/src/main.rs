#![allow(non_snake_case)]
#![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]

// VISP
use visp_common;

// Dioxus
use dioxus_desktop::{Config, WindowBuilder};

use std::fs::{self};
use platform_dirs::AppDirs;

fn main() {
    let app_dirs = AppDirs::new(Some("Quaint Studios/VISP"), true).unwrap();

    fs::create_dir_all(&app_dirs.data_dir).unwrap();

    dioxus_desktop::launch_cfg(visp_common::app, Config::new()
        .with_data_directory(&app_dirs.data_dir)
        .with_resource_directory(app_dirs.data_dir)
        .with_window(WindowBuilder::new().with_title("VISP - Home"))
        .with_custom_head(r#"<link rel="stylesheet" type="text/css" href="public/tailwind.css">"#.into()))
}