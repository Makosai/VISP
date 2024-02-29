use std::hash::Hasher;
use dioxus::prelude::*;

#[component]
pub(in crate::routes) fn Video(cx: Scope) -> Element {
    let files_uploaded: &UseRef<Vec<String>> = use_ref(cx, Vec::new);

    render! {
        div {
            class: "page",
            h1 {
                class: "font-black text-2xl",
                "Video"
            }
            div {
                class: "flex slot-70 border-black border-2",
                div {
                    class: "slot-30",
                    p { "30% | Info Windows" }
                    input {
                        // tell the input to pick a file
                        r#type:"file",
                        // list the accepted extensions
                        accept: ".mp4,.mov,.avi,.wmv,.flv,.f4v,.mkv,.webm,.mpeg-2,.3gp,.3g2,.ogv,.m4v",
                        // pick multiple files
                        multiple: true,
                        onchange: move |evt| {
                            // A helper macro to use hooks in async environments
                            to_owned![files_uploaded];
                            async move {
                                if let Some(file_engine) = &evt.files {
                                    let files = file_engine.files();
                                    for file_name in &files {
                                        // Make sure to use async/await when doing heavy I/O operations,
                                        // to not freeze the interface in the meantime
                                        if let Some(file) = file_engine.read_file_to_string(file_name).await{
                                            files_uploaded.write().push(file);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                div {
                    class: "slot-70",
                    p { "70% | Video & Effects Previews" }
                }
            }
            div {
                class: "flex flex-col slot-30 border-black border-2",
                div {
                    class: "slot-10",
                    p { "10% | Toolbar" }
                }
                div {
                    class: "slot-90",
                    p { "90% | Timeline" }
                }
            }
        }
}
}

pub fn upload_file() {}