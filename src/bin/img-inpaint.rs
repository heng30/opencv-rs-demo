use anyhow::Result;
use opencv::{core, highgui, imgcodecs, imgproc, prelude::*};

fn main() -> Result<()> {
    let (w, h) = (640, 480);
    let window_name = "img-inpaint";

    let mut img = opencv::imgcodecs::imread("data/fruit.png", imgcodecs::IMREAD_COLOR)?;
    let mut mask = opencv::imgcodecs::imread("data/fruit-mask.png", imgcodecs::IMREAD_COLOR)?;

    opencv::imgproc::resize(
        &img.clone(),
        &mut img,
        core::Size::new(w, h),
        0.,
        0.,
        opencv::imgproc::INTER_AREA,
    )?;

    opencv::imgproc::resize(
        &mask.clone(),
        &mut mask,
        core::Size::new(w, h),
        0.,
        0.,
        opencv::imgproc::INTER_AREA,
    )?;

    imgproc::cvt_color(
        &mask.clone(),
        &mut mask,
        imgproc::COLOR_BGR2GRAY,
        0,
        opencv::core::AlgorithmHint::ALGO_HINT_DEFAULT.into(),
    )?;
    mask.clone().convert_to_def(&mut mask, core::CV_8UC1)?;

    // 由于img和mask有点对不齐，效果有点差
    opencv::photo::inpaint(
        &img.clone(),
        &mask,
        &mut img,
        5.,
        opencv::photo::INPAINT_TELEA,
    )?;

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
