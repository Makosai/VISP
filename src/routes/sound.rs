use std::sync::Arc;
use dioxus::html::{FileEngine, HasFileData};
use dioxus::prelude::*;

#[cfg(target_family = "windows")]
use dioxus::desktop::use_asset_handler;
#[cfg(target_family = "windows")]
use dioxus::desktop::wry::http::Response;

#[cfg(target_family = "wasm")]
use dioxus::web::WebFileEngineExt;

#[cfg(target_family = "windows")]
async fn get_url(engine: Arc<dyn FileEngine>, file_name: &str) -> String {
    let file = engine.read_file(file_name).await.unwrap();

    // Use asset handler
    use_asset_handler("testing.mp4", move |request, response| {
        // We get the original path - make sure you handle that!
        if request.uri().path() != "/testing.mp4" {
            println!("{}", request.uri().path());
            return;
        }

        response.respond(Response::new(file.clone()));
    });

    return file_name.to_string();
}

#[cfg(target_family = "wasm")]
async fn get_url(engine: std::sync::Arc<dyn FileEngine>, file_name: &str) -> String {
    let file = engine.get_web_file(&*file_name).await.unwrap();
    return web_sys::Url::create_object_url_with_blob(&*file).unwrap();
}


#[component]
pub(in crate::routes) fn Sound() -> Element {
    let mut enable_directory_upload = use_signal(|| false);
    let mut files_uploaded_url = use_signal(|| Vec::new());

    let upload_files = move |evt: FormEvent| async move {
        for file_name in evt.files().unwrap().files() {
            // no files on form inputs?
            // sleep(std::time::Duration::from_secs(1)).await;

            #[cfg(target_family = "windows")]
            let object_url = get_url(evt.files().unwrap(), &*file_name).await;


            #[cfg(target_family = "wasm")]
                let object_url = get_url(evt.files().unwrap(), &*file_name).await;

            // Push the object URL to the files_uploaded_url signal
            files_uploaded_url.write().push(object_url);
        }
    };

    let handle_file_drop = move |evt: DragEvent| async move {
        if let Some(file_engine) = &evt.files() {
            let files = file_engine.files();
            for file_name in &files {
                let object_url = get_url(evt.files().unwrap(), file_name).await;

                // Push the object URL to the files_uploaded_url signal
                files_uploaded_url.write().push(object_url);
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
            for object_url in files_uploaded_url.read().iter() {
                li {
                    // Video Element with object URL source
                    "hello"
                    video {
                        controls: false,
                        src: "/{object_url}",
                        "Your browser does not support the video element."
                    }
                }
            }
        }
    }
}