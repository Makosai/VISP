use dioxus::html::HasFileData;
use dioxus::prelude::*;

#[component]
pub(in crate::routes) fn Sound() -> Element {
    let mut enable_directory_upload = use_signal(|| false);
    let mut file_names_uploaded = use_signal(|| Vec::new());
    let mut files_uploaded = use_signal(|| Vec::new());
    let mut files_uploaded_url = use_signal(|| Vec::new());

    let upload_files = move |evt: FormEvent| async move {
        for file_name in evt.files().unwrap().files() {
            // no files on form inputs?
            // sleep(std::time::Duration::from_secs(1)).await;
            // When uploaded, read the video file's contents
            if let Some(file) = evt.files().unwrap().read_file_to_string(&*file_name).await {
                // Convert file string to JSValue
                let js_value = web_sys::wasm_bindgen::JsValue::from_str(&file);

                // Create a File with u8_array_sequence data
                let blob_file = web_sys::File::new_with_str_sequence_and_options(
                    &js_value,
                    &file_name,
                    web_sys::FilePropertyBag::new().type_("video/mp4"),
                ).unwrap();

                // Create an Object URL for the video file
                let object_url = web_sys::Url::create_object_url_with_blob(&*blob_file).unwrap();

                // Push the object URL to the files_uploaded_url signal
                files_uploaded_url.write().push(object_url);
            }
            file_names_uploaded.write().push(file_name);
        }
    };

    let handle_file_drop = move |evt: DragEvent| async move {
        if let Some(file_engine) = &evt.files() {
            let files = file_engine.files();
            for file_name in &files {
                if let Some(file) = file_engine.read_file_to_string(file_name).await {
                    files_uploaded.write().push(file);
                }
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
                    video {
                        controls: true,
                        src: "{object_url}",
                        "Your browser does not support the video element."
                    }
                }
            }
        }
    }
}