mod home;
mod image;
mod not_found;
mod post;
mod sound;
mod video;

use crate::routes::home::*;
use crate::routes::image::*;
use crate::routes::not_found::*;
use crate::routes::post::*;
use crate::routes::sound::*;
use crate::routes::video::*;

use crate::components::nav_bar::NavBar;

use dioxus::prelude::*;

// https://dioxuslabs.com/learn/0.4/router/example/full-code
#[derive(Routable, PartialEq, Debug, Clone)]
pub enum Route {
    #[layout(NavBar)] #[route("/")] Home {},
    #[route("/video")] Video {},
    #[route("/image")] Image {},
    #[route("/sound")] Sound {},
    #[route("/post")] Post {},
    #[end_layout] //  if the current location doesn't match any of the above routes, render the NotFound component
    #[route("/:..route")] NotFound {
        route: Vec<String>,
    },
}
