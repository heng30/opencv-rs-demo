use anyhow::Result;
use opencv::{highgui, imgcodecs, imgproc::ColorConversionCodes};

fn main() -> Result<()> {
    let (w, h) = (640, 480);
    let window_name = "color";
    let trackbar_name = "ColorSpace";

    let color_space = [
        ColorConversionCodes::COLOR_BGR2BGRA,
        ColorConversionCodes::COLOR_BGR2GRAY,
        ColorConversionCodes::COLOR_BGR2RGB,
        ColorConversionCodes::COLOR_BGR2Luv,
        ColorConversionCodes::COLOR_BGR2YUV,
        ColorConversionCodes::COLOR_BGR2XYZ,
        ColorConversionCodes::COLOR_BGR2Lab,
        ColorConversionCodes::COLOR_BGR2YCrCb,
        ColorConversionCodes::COLOR_BGR2HSV_FULL,
        ColorConversionCodes::COLOR_RGB2HLS_FULL,
    ];

    highgui::named_window(window_name, highgui::WINDOW_NORMAL)?;
    highgui::resize_window(window_name, w, h)?;

    highgui::create_trackbar(
        trackbar_name,
        window_name,
        None,
        (color_space.len() - 1) as i32,
        Some(Box::new(move |_v| {})),
    )?;

    let mut dst_img = opencv::core::Mat::default();
    let src_img = imgcodecs::imread("color.png", imgcodecs::IMREAD_COLOR)?;

    loop {
        let pos = highgui::get_trackbar_pos(trackbar_name, window_name)? as usize;
        opencv::imgproc::cvt_color(
            &src_img,
            &mut dst_img,
            color_space[pos].into(),
            0,
            // opencv::core::AlgorithmHint::ALGO_HINT_DEFAULT.into(),
        )?;

        highgui::imshow(window_name, &dst_img)?;

        let key = highgui::wait_key(40)?;
        if key & 0xFF == 'q' as i32 {
            break;
        }
    }

    highgui::destroy_all_windows()?;
    Ok(())
}
