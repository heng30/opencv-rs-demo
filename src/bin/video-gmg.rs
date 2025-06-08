use anyhow::Result;
use opencv::{highgui, prelude::*};

fn main() -> Result<()> {
    let window_name = "video-gmg";

    let mut cap = opencv::videoio::VideoCapture::from_file(
        "data/running-cars.mp4",
        opencv::videoio::CAP_ANY,
    )?;
    let fps = cap.get(opencv::videoio::CAP_PROP_FPS)?;
    let delay = (1000.0 / fps) as i32; // ms per frame
    let w = cap.get(opencv::videoio::CAP_PROP_FRAME_WIDTH)? as i32;
    let h = cap.get(opencv::videoio::CAP_PROP_FRAME_HEIGHT)? as i32;

    // 去背景比mog2好，但是消耗更长时间. 也能够识别阴影，更少噪点
    let mut gmg = opencv::bgsegm::create_background_subtractor_gmg_def()?;

    highgui::named_window(window_name, highgui::WINDOW_NORMAL)?;
    highgui::resize_window(window_name, w, h)?;

    loop {
        let start = std::time::Instant::now();

        let mut frame = Mat::default();
        if let Ok(true) = cap.read(&mut frame) {
            gmg.apply(&frame.clone(), &mut frame, -1.)?;
            highgui::imshow(window_name, &frame)?;
        }

        let elapsed = start.elapsed().as_millis() as i32;
        let wait_time = std::cmp::max(1, delay - elapsed);

        let key = highgui::wait_key(wait_time)?;
        if key & 0xFF == 'q' as i32 {
            break;
        }
    }

    highgui::destroy_all_windows()?;
    Ok(())
}
