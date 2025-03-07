use dioxus::prelude::*;

#[component]
pub(in crate::routes) fn NotFound(route: Vec<String>) -> Element {
    rsx! {
        document::Title { "VISP - Page not found" }
        
        h1 { "Page not found" }
        p { "We are terribly sorry, but the page you requested doesn't exist." }
        pre { color: "red", "log:\nattemped to navigate to: {route:?}" }
    }
}
