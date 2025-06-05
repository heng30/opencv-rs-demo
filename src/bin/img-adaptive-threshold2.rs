use anyhow::Result;
use opencv::{core, highgui, imgcodecs};

fn main() -> Result<()> {
    let (w, h) = (640, 480);
    let window_name = "img-adaptive-threshold2";

    let img = imgcodecs::imread("data/math.png", imgcodecs::IMREAD_COLOR)?;

    let mut result = core::Mat::default();
    opencv::imgproc::cvt_color(
        &img,
        &mut result,
        opencv::imgproc::ColorConversionCodes::COLOR_BGR2GRAY.into(),
        0,
        // opencv::core::AlgorithmHint::ALGO_HINT_DEFAULT.into(),
    )?;

    opencv::imgproc::adaptive_threshold(
        &result.clone(),
        &mut result,
        255.,
        opencv::imgproc::AdaptiveThresholdTypes::ADAPTIVE_THRESH_GAUSSIAN_C.into(),
        opencv::imgproc::THRESH_BINARY,
        9,
        0.,
    )?;

    // 降噪
    let kernel = opencv::imgproc::get_structuring_element(
        opencv::imgproc::MorphShapes::MORPH_CROSS.into(),
        core::Size::new(3, 3),
        core::Point::new(-1, -1),
    )?;

    opencv::imgproc::morphology_ex(
        &result.clone(),
        &mut result,
        opencv::imgproc::MORPH_OPEN,
        &kernel,
        core::Point::new(-1, -1),
        1,
        core::BORDER_CONSTANT,
        opencv::imgproc::morphology_default_border_value()?,
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
