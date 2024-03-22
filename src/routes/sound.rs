use std::sync::Arc;
use dioxus::html::{FileEngine, HasFileData};
use dioxus::prelude::*;

use gstreamer as gst;
use gstreamer::prelude::*;
use gstreamer_app as gst_app;
use gstreamer::MessageView;

#[cfg(target_family = "windows")]
use dioxus::desktop::use_asset_handler;
#[cfg(target_family = "windows")]
use dioxus::desktop::wry::http::Response;

#[cfg(target_family = "wasm")]
use dioxus::web::WebFileEngineExt;

#[cfg(target_family = "windows")]
async fn get_video_file(engine: Arc<dyn FileEngine>, file_name: &str) -> VideoFile {
    let file = engine.read_file(file_name).await.unwrap();
    let file_clone = file.clone();

    // Use asset handler
    use_asset_handler("testing.mp4", move |request, response| {
        // We get the original path - make sure you handle that!
        if request.uri().path() != "/testing.mp4" {
            println!("{}", request.uri().path());
            return;
        }

        response.respond(Response::new(file_clone.to_owned()));
    });

    return VideoFile::new(file_name.to_string(), file_name.to_string(), 8 * 1_000_000_000, file);
}

#[cfg(target_family = "wasm")]
async fn get_video_file(engine: std::sync::Arc<dyn FileEngine>, file_name: &str) -> VideoFile {
    let web_file = engine.get_web_file(&*file_name).await.unwrap();

    let file = engine.read_file().unwrap();
    let object_url = web_sys::Url::create_object_url_with_blob(&*web_file).unwrap();

    VideoFile::new(object_url, file_name.to_string(), file);
}

pub struct VideoFile {
    object_url: String,
    file_name: String,
    duration: i64, // Nanoseconds
    data: Vec<u8>,
}

impl VideoFile {
    pub fn new(object_url: String, file_name: String, duration: i64, data: Vec<u8>) -> Self {
        Self { object_url, file_name, duration, data }
    }
}

/*
fn cut_video_gstreamer(input_bytes: Vec<u8>, duration: i64, output_bytes: &mut Vec<u8>) -> Result<(), gstreamer::FlowError> {
    gst::init().unwrap();

    // Now we can calculate half_duration
    let half_duration = duration / 2;

    let pipeline = gst::Pipeline::with_name("video_cutter");

    // Create elements
    let appsrc = gst_app::AppSrc::builder().name("appsrc").build().context("Failed to create appsrc");
    let capsfilter = gst::ElementFactory::make("capsfilter", None).context("Failed to create capsfilter")?;
    let videocut = gst::ElementFactory::make("videocut").context("Failed to create videocut")?;

    // Consider using filesink instead of autovideosink for testing
    let sink = gst::ElementFactory::make("filesink");
    sink.set_property("location", &"output.mp4"); // Adjust output file name

    // Set up capsfilter (adjust for your video format)
    let caps_str = "video/x-raw,format=RGB,width=640,height=480,framerate=30/1";
    let caps = gst::Caps::from_string(caps_str).unwrap();
    capsfilter.set_property("caps", &caps).unwrap();

    // Add elements and link
    pipeline.add_many(&[&appsrc, &capsfilter, &videocut, &sink]).unwrap();
    appsrc.link(&capsfilter).unwrap();
    videocut.connect_pad_dynamic("src", &sink.static_pad("sink").unwrap()).unwrap();

    // ... (Calculate mid-point of the video duration like before) ...

    // Set start and end times for cut
    videocut.set_property("start-time", &0 * gst::ClockTime::SECOND).unwrap();
    videocut.set_property("end-time", &half_duration * gst::ClockTime::SECOND).unwrap();

    // Feed input bytes (assuming input_bytes is a full video)
    let buffer = gst::Buffer::from_mut_slice(&input_bytes);
    appsrc.push_buffer(buffer)?;

    // Start the pipeline
    pipeline.set_state(gst::State::Playing)?;

    // Basic bus handling
    let bus = pipeline.bus().unwrap();
    for msg in bus.iter_timed(gst::ClockTime::NONE) {
        match msg.view() {
            MessageView::Eos(..) => break, // End-of-stream
            MessageView::Error(err) => {
                println!("Error: {} ({})", err.error(), err.debug().unwrap_or("None"));
                break;
            }
            _ => (),
        }
    }

    // Stop the pipeline and collect output (if desired)
    pipeline.set_state(gst::State::Null)?;

    // At this point, your 'output.mp4' file should contain the result
    // read the output a Vec<u8>

    Ok(())

}
*/

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
                let object_url = get_video_file(evt.files().unwrap(), &*file_name).await;

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