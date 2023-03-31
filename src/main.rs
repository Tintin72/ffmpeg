extern crate video_rs;

use video_rs::{Buf, Locator, Reader, Write, Writer, Decoder};
use std::path::{PathBuf, Path};

fn main() {
    let path = "/home/martinkinaro/Videos/test-cam.mkv";
    let path_buf = PathBuf::from(&path);
    let locator_path = Locator::Path(path_buf);
    let decoder = Decoder::new(&locator_path);
    println!("Hello, {}", path);
}
