use anyhow::Result;
use opencv::{highgui, imgcodecs, prelude::*};

fn main() -> Result<()> {
    let (w, h) = (640, 480);
    let window_name = "img-multi-match-template";

    let mut img_orig = imgcodecs::imread("data/mario.png", imgcodecs::IMREAD_COLOR)?;
    let mut img = Mat::default();

    opencv::imgproc::cvt_color(
        &img_orig,
        &mut img,
        opencv::imgproc::ColorConversionCodes::COLOR_BGR2GRAY.into(),
        0,
        opencv::core::AlgorithmHint::ALGO_HINT_DEFAULT.into(),
    )?;

    let mut img_template = imgcodecs::imread("data/mario-coin.png", imgcodecs::IMREAD_COLOR)?;

    opencv::imgproc::cvt_color(
        &img_template.clone(),
        &mut img_template,
        opencv::imgproc::ColorConversionCodes::COLOR_BGR2GRAY.into(),
        0,
        opencv::core::AlgorithmHint::ALGO_HINT_DEFAULT.into(),
    )?;

    let mut result = Mat::default();
    opencv::imgproc::match_template_def(
        &img,
        &img_template,
        &mut result,
        opencv::imgproc::TM_CCOEFF_NORMED,
    )?;

    // println!("{:?}", result);

    let threshold = 0.7;
    let template_size = img_template.size()?;

    for i in 0..result.rows() {
        for j in 0..result.cols() {
            if *result.at_2d::<f32>(i, j)? < threshold {
                continue;
            }

            opencv::imgproc::rectangle(
                &mut img_orig,
                opencv::core::Rect::new(j, i, template_size.width, template_size.height),
                opencv::core::Scalar::new(0., 0., 255., 0.),
                1,
                opencv::imgproc::LINE_4,
                0,
            )?;
        }
    }

    highgui::named_window(window_name, highgui::WINDOW_NORMAL)?;
    highgui::resize_window(window_name, w, h)?;
    highgui::imshow(window_name, &img_orig)?;

    loop {
        let key = highgui::wait_key(0)?;
        if key & 0xFF == 'q' as i32 {
            break;
        }
    }

    highgui::destroy_all_windows()?;
    Ok(())
}
