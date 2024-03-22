use std::sync::Arc;
use dioxus::html::{FileEngine, HasFileData};
use dioxus::prelude::*;
use crate::video::manager::*;
use crate::video::VideoFile;

#[component]
pub(in crate::routes) fn Sound() -> Element {
    let mut enable_directory_upload = use_signal(|| false);
    let mut video_files_uploaded = use_signal(|| Vec::new() as Vec<VideoFile>);

    let upload_files = move |evt: FormEvent| async move {
        for file_name in evt.files().unwrap().files() {
            // no files on form inputs?
            // sleep(std::time::Duration::from_secs(1)).await;

            #[cfg(target_family = "windows")]
            let video_file = get_video_file(evt.files().unwrap(), &*file_name).await;

            #[cfg(target_family = "wasm")]
            let video_file = get_video_file(evt.files().unwrap(), &*file_name).await;

            // Push the object URL to the files_uploaded_url signal
            video_files_uploaded.write().push(video_file);
        }
    };

    let handle_file_drop = move |evt: DragEvent| async move {
        if let Some(file_engine) = &evt.files() {
            let files = file_engine.files();
            for file_name in &files {
                let video_file = get_video_file(evt.files().unwrap(), file_name).await;

                // Push the object URL to the files_uploaded_url signal
                video_files_uploaded.write().push(video_file);
            }
        }
    };

    rsx! {
        input {
            r#type: "checkbox",
            id: "directory-upload",
            checked: enable_directory_upload,
            oninput: move |evt| enable_directory_upload.set(evt.checked()),
        },
        label {
            r#for: "directory-upload",
            "Enable directory upload"
        }

        input {
            r#type: "file",
            accept: "video/*",
            multiple: true,
            directory: enable_directory_upload,
            onchange: upload_files,
        }

        div {
            // cheating with a little bit of JS...
            "ondragover": "this.style.backgroundColor='#88FF88';",
            "ondragleave": "this.style.backgroundColor='#FFFFFF';",

            id: "drop-zone",
            prevent_default: "ondrop dragover dragenter",
            ondrop: handle_file_drop,
            ondragover: move |event| event.stop_propagation(),
            "Drop files here"
        }
        ul {
            for video_file in video_files_uploaded.read().iter() {
                li {
                    // Video Element with object URL source
                    "hello"
                    video {
                        controls: true,
                        src: "/{video_file.object_url}",
                        "Your browser does not support the video element."
                    }
                }
            }
        }
    }
}