#[cfg_attr(target_family = "windows", path = "desktop_video_manager.rs")]
#[cfg_attr(target_family = "unix", path = "desktop_video_manager.rs")]
#[cfg_attr(target_family = "wasm", path = "web_video_manager.rs")]
pub mod manager;

pub struct VideoFile {
    pub object_url: String,
    pub file_name: String,
    pub duration: u64, // Nanoseconds
    pub data: Vec<u8>,
}

impl VideoFile {
    pub fn new(object_url: String, file_name: String, duration: u64, data: Vec<u8>) -> Self {
        Self { object_url, file_name, duration, data }
    }
}