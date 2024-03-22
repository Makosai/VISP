use std::sync::Arc;
use dioxus::html::{FileEngine, HasFileData};
use dioxus::prelude::*;
use crate::video::manager::*;
use crate::video::VideoFile;

#[component]
pub(in crate::routes) fn Sound() -> Element {

    rsx! {
        h1 {
            class: "font-black text-2xl",
            "Sound"
        }
    }
}