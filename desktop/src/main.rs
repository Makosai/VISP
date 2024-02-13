#![allow(non_snake_case)]
#![cfg_attr(feature = "bundle", windows_subsystem = "windows")]

use visp_common;

use dioxus::prelude::*;
use dioxus_desktop::Config;

fn main() {
    dioxus_desktop::launch_cfg(visp_common::app, Config::new()
        .with_custom_head(r#"<link rel="stylesheet" type="text/css" href="public/tailwind.css">"#.into()))
}