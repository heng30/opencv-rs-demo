use anyhow::Result;
use opencv::highgui;
use opencv::prelude::*;

fn main() -> Result<()> {
    let (w, h) = (640, 480);
    let mut img = Mat::zeros(h, w, opencv::core::CV_8UC3)?.to_mat()?;

    opencv::imgproc::rectangle(
        &mut img,
        opencv::core::Rect::new(50, 50, 300, 300),
        opencv::core::Scalar::new(0., 255., 255., 0.),
        2,
        opencv::imgproc::LINE_AA,
        0,
    )?;

    let window_name = "draw-rectangle";
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
