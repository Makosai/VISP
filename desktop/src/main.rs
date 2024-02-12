#![cfg_attr(feature = "bundle", windows_subsystem = "windows")]

use visp_common::start_app;

fn main() {
    start_app(visp_common::app);
}