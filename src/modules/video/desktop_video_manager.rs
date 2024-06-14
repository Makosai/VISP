use std::sync::Arc;
use dioxus::desktop::use_asset_handler;
use dioxus::desktop::wry::http::Response;
use dioxus::html::FileEngine;
use crate::video::VideoFile;

pub(crate) async fn get_video_file(engine: Arc<dyn FileEngine>, file_name: &str) -> VideoFile {
    let file = engine.read_file(file_name).await.unwrap();
    let file_clone = trim_video_data(file.clone(), Duration::new(1, 0).as_nanos() as u64).unwrap();

    // Use asset handler
    use_asset_handler("testing.mp4", move |request, response| {
        // We get the original path - make sure you handle that!
        if request.uri().path() != "/testing.mp4" {
            println!("{}", request.uri().path());
            return;
        }

        response.respond(Response::new(file_clone.to_owned()));
    });

    return VideoFile::new(file_name.to_string(), file_name.to_string(), Duration::new(1, 0).as_nanos() as u64, file);
}

use gstreamer as gst;
use gstreamer::prelude::*;
use gstreamer_app as gst_app;
use gstreamer_video as gst_video;
use std::sync::Mutex;
use std::time::Duration;

fn trim_video_data(input_data: Vec<u8>, duration: u64) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    // Initialize GStreamer
    gst::init().unwrap();

    // Create the pipeline
    let pipeline = gst::Pipeline::with_name("video_cutter");

    // Create the elements
    let appsrc = gst_app::AppSrc::builder()
        .build();

    let caps = gst_video::VideoCapsBuilder::new()
        .width(400)
        .height(300)
        .framerate((15, 1).into())
        .format(gst_video::VideoFormat::Rgb)
        .build();
    let capsfilter = gst::ElementFactory::make("capsfilter")
        .property("caps", &caps)
        .build()?;

    let videoconvert = gst::ElementFactory::make("videoconvert").build().unwrap();
    let x264enc = gst::ElementFactory::make("x264enc").build().unwrap();
    let mp4mux = gst::ElementFactory::make("mp4mux").build().unwrap();

    let appsink = gst_app::AppSink::builder().build();

    pipeline.add_many(&[appsrc.upcast_ref(), &capsfilter, &videoconvert, &x264enc, &mp4mux, &appsink.upcast_ref()]).unwrap();
    gst::Element::link_many(&[appsrc.upcast_ref(), &capsfilter, &videoconvert, &x264enc, &mp4mux, &appsink.upcast_ref()]).unwrap();

    let buffer = gst::Buffer::from_mut_slice(input_data);
    appsrc.push_buffer(buffer).unwrap();

    let output_data = Arc::new(Mutex::new(Vec::new()));
    let output_data_clone = Arc::clone(&output_data);
    appsink.set_callbacks(
        gst_app::AppSinkCallbacks::builder()
            .new_sample(move |appsink| {
                let sample = appsink.pull_sample().map_err(|_| gst::FlowError::Eos)?;
                let buffer = sample.buffer().ok_or_else(|| {
                    gstreamer::element_error!(
                        appsink,
                        gst::ResourceError::Failed,
                        ("Failed to get buffer from appsink")
                    );

                    gst::FlowError::Error
                })?;
                let map = buffer.map_readable().unwrap();
                output_data_clone.lock().unwrap().extend_from_slice(map.as_slice());
                Ok(gst::FlowSuccess::Ok)
            })
            .build(),
    );

    pipeline.set_state(gst::State::Playing).unwrap();

    let bus = pipeline.bus().unwrap();
    let timeout = gst::ClockTime::from_nseconds(duration);
    let clock = gst::SystemClock::obtain();
    let start_time = clock.time();
    while let Some(msg) = bus.timed_pop(timeout) {
        use gst::MessageView;
        match msg.view() {
            MessageView::Eos(..) => break,
            MessageView::Error(err) => {
                eprintln!(
                    "Error from {:?}: {} ({:?})",
                    err.src().map(|s| s.path_string()),
                    err.error(),
                    err.debug()
                );
                break;
            }
            _ => (),
        }
        if clock.time().unwrap() - start_time.unwrap() >= gst::ClockTime::from_nseconds(duration) {
            pipeline.send_event(gst::event::Eos::new());
        }
    }

    pipeline.set_state(gst::State::Null).unwrap();

    let output_vec = output_data.lock().unwrap().to_vec();
    Ok(output_vec)
}