use anyhow::Result;
use opencv::{core, highgui, imgcodecs, prelude::*};

fn main() -> Result<()> {
    let (w, h) = (640, 480);
    let window_name = "img-filter-blur";

    let img = imgcodecs::imread("data/color.png", imgcodecs::IMREAD_COLOR)?;

    // blur
    let kernel = (core::Mat::ones(5, 5, core::CV_32F)?.to_mat()? / 25.).into_result()?;

    let mut result = core::Mat::default();
    opencv::imgproc::filter_2d(
        &img,
        &mut result,
        -1,
        &kernel,
        core::Point::new(-1, -1),
        0.,
        core::BORDER_DEFAULT,
    )?;

    highgui::named_window(window_name, highgui::WINDOW_NORMAL)?;
    highgui::resize_window(window_name, w, h)?;

    loop {
        highgui::imshow(window_name, &result)?;

        let key = highgui::wait_key(0)?;
        if key & 0xFF == 'q' as i32 {
            break;
        }
    }

    highgui::destroy_all_windows()?;
    Ok(())
}
