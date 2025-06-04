use anyhow::Result;
use opencv::{core, highgui, imgcodecs, prelude::*};

fn main() -> Result<()> {
    let (w, h) = (640, 480);
    let logo_size = 200;
    let window_name = "img-logo";

    // make logo
    let mut logo = core::Mat::zeros(logo_size, logo_size, opencv::core::CV_8UC3)?.to_mat()?;

    opencv::imgproc::rectangle(
        &mut logo,
        opencv::core::Rect::new(30, 30, 100, 100),
        opencv::core::Scalar::new(0., 0., 255., 0.),
        -1,
        opencv::imgproc::LINE_4,
        0,
    )?;

    opencv::imgproc::rectangle(
        &mut logo,
        opencv::core::Rect::new(70, 70, 100, 100),
        opencv::core::Scalar::new(0., 255., 0., 0.),
        -1,
        opencv::imgproc::LINE_4,
        0,
    )?;

    // make mask
    let mut mask = Mat::new_rows_cols_with_default(
        logo_size,
        logo_size,
        opencv::core::CV_8UC3,
        core::Scalar::all(255.),
    )?;

    opencv::imgproc::rectangle(
        &mut mask,
        opencv::core::Rect::new(30, 30, 100, 100),
        opencv::core::Scalar::new(0., 0., 0., 0.),
        -1,
        opencv::imgproc::LINE_4,
        0,
    )?;

    opencv::imgproc::rectangle(
        &mut mask,
        opencv::core::Rect::new(70, 70, 100, 100),
        opencv::core::Scalar::new(0., 0., 0., 0.),
        -1,
        opencv::imgproc::LINE_4,
        0,
    )?;

    // make roi
    let mut img = imgcodecs::imread("test.png", imgcodecs::IMREAD_COLOR)?;

    let logo_pos = core::Rect::new(50, 50, logo_size, logo_size);
    let mut img_roi = core::Mat::roi_mut(&mut img, logo_pos)?;

    let mut img_roi_result = core::Mat::default();
    core::bitwise_and(&img_roi, &mask, &mut img_roi_result, &core::no_array())?;

    // mask final logo
    let mut logo_result = core::Mat::default();
    core::bitwise_or(&logo, &img_roi_result, &mut logo_result, &core::no_array())?;

    // copy `logo_result`, that is a sub matix of `img`, to the `img` at the `logo_pos`
    logo_result.copy_to(&mut img_roi)?;

    highgui::named_window(window_name, highgui::WINDOW_NORMAL)?;
    highgui::resize_window(window_name, w, h)?;

    loop {
        highgui::imshow(window_name, &img)?;

        let key = highgui::wait_key(0)?;
        if key & 0xFF == 'q' as i32 {
            break;
        }
    }

    highgui::destroy_all_windows()?;
    Ok(())
}
