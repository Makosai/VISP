use dioxus::html::FileEngine;
use dioxus::prelude::*;
use dioxus::web::WebFileEngineExt;
use crate::video::VideoFile;

pub(crate) async fn get_video_file(engine: std::sync::Arc<dyn FileEngine>, file_name: &str) -> VideoFile {
    let web_file = engine.get_web_file(&*file_name).await.unwrap();

    let file = engine.read_file(file_name).await.unwrap();
    let object_url = web_sys::Url::create_object_url_with_blob(&*web_file).unwrap();

    VideoFile::new(object_url, file_name.to_string(), 8 * 1_000_000_000, file)
}