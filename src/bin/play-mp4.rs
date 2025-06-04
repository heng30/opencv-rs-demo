use anyhow::Result;
use opencv::{core::Mat, highgui, prelude::*, videoio::VideoCapture};

fn main() -> Result<()> {
    let mut cap = VideoCapture::from_file("data/test.mp4", opencv::videoio::CAP_ANY)?;

    let fps = cap.get(opencv::videoio::CAP_PROP_FPS)?;
    let delay = (1000.0 / fps) as i32; // ms per frame

    let window_name = "play-mp4";
    highgui::named_window(&window_name, highgui::WINDOW_NORMAL)?;

    // Optionally resize window to video dimensions
    let width = cap.get(opencv::videoio::CAP_PROP_FRAME_WIDTH)? as i32;
    let height = cap.get(opencv::videoio::CAP_PROP_FRAME_HEIGHT)? as i32;
    highgui::resize_window(&window_name, width, height)?;

    let mut img = Mat::default();
    loop {
        let start = std::time::Instant::now();

        if let Ok(true) = cap.read(&mut img) {
            highgui::imshow(window_name, &img)?;
        }

        // Calculate processing time and adjust wait time
        let elapsed = start.elapsed().as_millis() as i32;
        let wait_time = std::cmp::max(1, delay - elapsed);

        let key = highgui::wait_key(wait_time)?;
        if key & 0xFF == 'q' as i32 {
            break;
        }
    }

    cap.release()?;
    highgui::destroy_all_windows()?;

    Ok(())
}
