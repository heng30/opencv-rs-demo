use anyhow::Result;
use opencv::{core, highgui, imgcodecs};

fn main() -> Result<()> {
    let (w, h) = (640, 480);
    let window_name = "img-threshold";

    let img = imgcodecs::imread("data/color.png", imgcodecs::IMREAD_COLOR)?;

    let mut result = core::Mat::default();
    opencv::imgproc::cvt_color(
        &img,
        &mut result,
        opencv::imgproc::ColorConversionCodes::COLOR_BGR2GRAY.into(),
        0,
        // opencv::core::AlgorithmHint::ALGO_HINT_DEFAULT.into(),
    )?;

    opencv::imgproc::threshold(
        &result.clone(),
        &mut result,
        160.,
        255.,
        opencv::imgproc::THRESH_BINARY,
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
