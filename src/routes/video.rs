use dioxus::html::HasFileData;
use dioxus::prelude::*;
use crate::video::manager::get_video_file;
use crate::video::VideoFile;

#[component]
pub(in crate::routes) fn Video() -> Element {
    let mut enable_directory_upload = use_signal(|| false);
    let mut video_files_uploaded = use_signal(|| Vec::new() as Vec<VideoFile>);

    let upload_files = move |evt: FormEvent| async move {
        for file_name in evt.files().unwrap().files() {
            // no files on form inputs?
            // sleep(std::time::Duration::from_secs(1)).await;

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
                    div {
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
                    }
                }
                div {
                    class: "slot-70",
                    p { "70% | Video & Effects Previews" }
                    div {
                        ul {
                            for video_file in video_files_uploaded.read().iter() {
                                li {
                                    // Video Element with object URL source
                                    p { "{video_file.file_name}" }
                                    video {
                                        controls: true,
                                        src: "/testing.mp4",
                                        "Your browser does not support the video element."
                                    }
                                }
                            }
                        }
                    }
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