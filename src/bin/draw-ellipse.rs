use anyhow::Result;
use opencv::highgui;
use opencv::prelude::*;

fn main() -> Result<()> {
    let (w, h) = (640, 480);
    let mut img = Mat::zeros(h, w, opencv::core::CV_8UC3)?.to_mat()?;

    opencv::imgproc::ellipse(
        &mut img,
        opencv::core::Point::new(200, 200),
        opencv::core::Size::new(150, 100),
        30.,
        0.,
        360.,
        opencv::core::Scalar::new(0., 255., 255., 0.),
        -1,
        opencv::imgproc::LINE_AA,
        0,
    )?;

    let window_name = "draw-ellipse";
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
