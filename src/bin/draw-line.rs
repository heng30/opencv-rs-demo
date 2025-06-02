use anyhow::Result;
use opencv::highgui;
use opencv::prelude::*;

fn main() -> Result<()> {
    let (w, h) = (640, 480);
    let mut img = Mat::zeros(h, w, opencv::core::CV_8UC3)?.to_mat()?;

    opencv::imgproc::line(
        &mut img,
        opencv::core::Point::new(50, 50),
        opencv::core::Point::new(300, 300),
        opencv::core::Scalar::new(0., 0., 255., 0.),
        4,
        opencv::imgproc::LINE_8,
        0,
    )?;

    opencv::imgproc::line(
        &mut img,
        opencv::core::Point::new(100, 50),
        opencv::core::Point::new(350, 300),
        opencv::core::Scalar::new(0., 255., 255., 0.),
        4,
        opencv::imgproc::LINE_AA,
        0,
    )?;

    let window_name = "draw-line";
    highgui::named_window(window_name, highgui::WINDOW_NORMAL)?;
    highgui::resize_window(window_name, w, h)?;

    highgui::imshow(window_name, &img)?;

    loop {
        let key = highgui::wait_key(0)?;
        if key & 0xFF == 'q' as i32 {
            break;
        }
    }

    highgui::destroy_all_windows()?;

    Ok(())
}
