use anyhow::Result;
use opencv::{core, highgui, imgcodecs};

fn main() -> Result<()> {
    let (w, h) = (640, 480);
    let window_name = "img-morphology3";

    let img = imgcodecs::imread("data/erode.png", imgcodecs::IMREAD_COLOR)?;

    let kernel = opencv::imgproc::get_structuring_element(
        opencv::imgproc::MorphShapes::MORPH_RECT.into(),
        core::Size::new(3, 3),
        core::Point::new(-1, -1),
    )?;

    let mut result = core::Mat::default();

    // 原图 - 腐蚀
    opencv::imgproc::morphology_ex(
        &img,
        &mut result,
        opencv::imgproc::MORPH_GRADIENT,
        &kernel,
        core::Point::new(-1, -1),
        2,
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
