// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder
        ::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, cut_video_in_half, rewrite_full_video])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

use gst::parse;
use gst::prelude::*;
use gst_pbutils::Discoverer;
use gstreamer as gst;
use gstreamer_pbutils as gst_pbutils;
use std::sync::{ atomic::{ AtomicBool, Ordering }, Arc };

///
/// This is a prototype to showcase that we can get GStreamer working in Tauri.
/// This tries to follow the "rewrite_full_video" example, but cuts the video in half.
///
/// Current problems:
/// - Seeking causes the application to freeze.
/// - It seems like any loops for a MessageView never gets a message. Not sure why.
/// - One band-aid was to add a timeout to the timed_pop calls, but that is not a real solution.
/// - The pipeline seems to run fine, but the seek does not work as expected.
///
#[tauri::command]
fn cut_video_in_half(input_path: String, output_path: String) -> Result<String, String> {
    gst::init().map_err(|e| format!("Failed to init GStreamer: {e}"))?;

    // Handle Windows paths
    let input_fs = input_path.replace('\\', "/");
    let output_fs = output_path.replace('\\', "/");

    // TODO: This is where we decode the video. We'd want to customize the bitrate for the audio and video.
    // TODO: Maybe ask around about making this efficient.
    let pipeline_str = format!(
        concat!(
            "filesrc location=\"{in_}\" ! decodebin name=dec ",
            "dec. ! queue ! videoconvert ! x264enc speed-preset=slow bitrate=3000 ! queue ! mux. ",
            "dec. ! queue ! audioconvert ! audioresample ! avenc_aac bitrate=128000 ! queue ! mux. ",
            "mp4mux name=mux faststart=true ! filesink location=\"{out}\""
        ),
        in_ = input_fs,
        out = output_fs
    );

    let pipeline = parse
        ::launch(&pipeline_str)
        .map_err(|e| format!("Failed to create pipeline: {e}"))?
        .downcast::<gst::Pipeline>()
        .map_err(|_| "Pipeline downcast failed")?;

    // Set to Paused and wait for preroll (AsyncDone)
    pipeline.set_state(gst::State::Paused).unwrap();
    let bus = pipeline.bus().unwrap();
    loop {
        use gst::MessageView;
        match bus.timed_pop(gst::ClockTime::from_seconds(240)) {
            Some(msg) =>
                match msg.view() {
                    MessageView::AsyncDone(_) => {
                        break;
                    }
                    MessageView::Error(err) => {
                        pipeline.set_state(gst::State::Null).ok();
                        return Err(format!("Preroll error: {}", err.error()));
                    }
                    _ => {}
                }
            None => {
                pipeline.set_state(gst::State::Null).ok();
                return Err("Timeout waiting for preroll".into());
            }
        }
    }

    // Query duration and compute midpoint
    let duration = pipeline.query_duration::<gst::ClockTime>().ok_or("Failed to query duration")?;
    if duration.is_none() || duration.nseconds() == 0 {
        pipeline.set_state(gst::State::Null).ok();
        return Err("Unknown duration".into());
    }
    let midpoint: gst::ClockTime = gst::ClockTime::from_nseconds(duration.nseconds() / 2);

    // Seek to the first half (0 to midpoint)
    let seek_flags = gst::SeekFlags::FLUSH | gst::SeekFlags::KEY_UNIT | gst::SeekFlags::ACCURATE;

    pipeline
        .seek(
            1.0, // rate
            seek_flags,
            gst::SeekType::Set,
            gst::ClockTime::ZERO,
            gst::SeekType::Set,
            midpoint
        )
        .map_err(|_| "Seek failed")?;

    // Set to Playing
    pipeline.set_state(gst::State::Playing).unwrap();

    // Listen for EOS or Error
    let mut result = Ok(());
    loop {
        use gst::MessageView;
        match bus.timed_pop(gst::ClockTime::from_seconds(240)) {
            Some(msg) =>
                match msg.view() {
                    MessageView::Eos(..) => {
                        break;
                    }
                    MessageView::Error(err) => {
                        result = Err(format!("Pipeline error: {}", err.error()));
                        break;
                    }
                    _ => {}
                }
            None => {
                result = Err("Timeout waiting for EOS".into());
                break;
            }
        }
    }
    pipeline.set_state(gst::State::Null).ok();

    result.map(|_| output_path).map_err(|e| e.to_string())
}

///
/// This is a prototype to showcase that we can get GStreamer working in Tauri.
/// This function rewtrites the input video to a new MP4 file. This acts as a base
/// for more complex editing operations.
///
#[tauri::command]
fn rewrite_full_video(input_path: String, output_path: String) -> Result<String, String> {
    gst::init().map_err(|e| format!("Failed to init GStreamer: {e}"))?;

    // Handle Windows paths
    let input_fs = input_path.replace('\\', "/");
    let output_fs = output_path.replace('\\', "/");

    // Remux MP4 (H.264 + AAC). Use request-pad linking "mux." and enforce caps.
    let pipeline_av = format!(
        concat!(
            "filesrc location=\"{in_}\" ! qtdemux name=demux ",
            // video branch -> mp4 needs avc/au
            "demux.video_0 ! queue ! h264parse config-interval=-1 ! ",
            "video/x-h264,stream-format=avc,alignment=au ! queue ! mux. ",
            // audio branch -> mp4 needs raw AAC
            "demux.audio_0 ! queue ! aacparse ! ",
            "audio/mpeg,mpegversion=4,stream-format=raw ! queue ! mux. ",
            // mux
            "mp4mux name=mux faststart=true ! filesink location=\"{out}\""
        ),
        in_ = input_fs,
        out = output_fs
    );

    // Fallback: video-only (no audio track)
    let pipeline_v = format!(
        concat!(
            "filesrc location=\"{in_}\" ! qtdemux name=demux ",
            "demux.video_0 ! queue ! h264parse config-interval=-1 ! ",
            "video/x-h264,stream-format=avc,alignment=au ! queue ! mux. ",
            "mp4mux name=mux faststart=true ! filesink location=\"{out}\""
        ),
        in_ = input_fs,
        out = output_fs
    );

    // Last-resort fallback: re-encode A/V so any input becomes MP4
    let pipeline_reencode = format!(
        concat!(
            "uridecodebin uri=\"file:///{in_}\" name=d ",
            "d. ! queue ! videoconvert ! ",
            "x264enc speed-preset=slow tune=zerolatency key-int-max=60 ! ",
            "h264parse config-interval=-1 ! video/x-h264,stream-format=avc,alignment=au ! queue ! mux. ",
            "d. ! queue ! audioconvert ! audioresample ! ",
            "avenc_aac bitrate=128000 ! aacparse ! ",
            "audio/mpeg,mpegversion=4,stream-format=raw ! queue ! mux. ",
            "mp4mux name=mux faststart=true ! filesink location=\"{out}\""
        ),
        in_ = input_fs,
        out = output_fs
    );

    let pipeline = (
        match parse::launch(&pipeline_av) {
            Ok(p) => p,
            Err(e1) => {
                eprintln!("Remux A+V failed: {e1}. Trying video-only remux…");
                match parse::launch(&pipeline_v) {
                    Ok(p) => p,
                    Err(e2) => {
                        eprintln!("Remux video-only failed: {e2}. Re-encoding…");
                        parse
                            ::launch(&pipeline_reencode)
                            .map_err(|e| format!("Failed to build pipeline: {e}"))?
                    }
                }
            }
        }
    )
        .downcast::<gst::Pipeline>()
        .map_err(|_| "Pipeline downcast failed")?;

    // Play until EOS
    pipeline.set_state(gst::State::Playing).unwrap();
    let bus = pipeline.bus().unwrap();
    let mut result = Ok(());
    while let Some(msg) = bus.timed_pop(None) {
        use gst::MessageView;
        match msg.view() {
            MessageView::Eos(..) => {
                break;
            }
            MessageView::Error(err) => {
                result = Err(format!("Pipeline error: {}", err.error()));
                break;
            }
            _ => {}
        }
    }
    pipeline.set_state(gst::State::Null).ok();

    result.map(|_| output_path).map_err(|e| e.to_string())
}
