use anyhow::Result;
use opencv::{core::Mat, highgui, prelude::*, videoio::VideoCapture};

fn main() -> Result<()> {
    let mut cap = VideoCapture::from_file("test.mp4", opencv::videoio::CAP_ANY)?;

    let window_name = "play-mp4";
    highgui::named_window(&window_name, highgui::WINDOW_NORMAL)?;
    highgui::resize_window(&window_name, 640, 480)?;

    let mut img = Mat::default();
    loop {
        if let Ok(true) = cap.read(&mut img) {
            highgui::imshow(window_name, &img)?;
        }

        let key = highgui::wait_key(40)?;
        if key & 0xFF == 'q' as i32 {
            break;
        }
    }

    cap.release()?;
    highgui::destroy_all_windows()?;

    Ok(())
}
