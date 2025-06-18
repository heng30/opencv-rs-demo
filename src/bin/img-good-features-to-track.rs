use anyhow::Result;
use opencv::{core, core::Mat, highgui, imgcodecs, imgproc, prelude::*};

fn main() -> Result<()> {
    let (w, h) = (640, 480);
    let window_name = "img-good-features-to-track";

    let mut img = imgcodecs::imread("data/chess.png", imgcodecs::IMREAD_COLOR)?;

    // 灰度
    let mut gray = Mat::default();
    imgproc::cvt_color(
        &img,
        &mut gray,
        imgproc::COLOR_BGR2GRAY,
        0,
        opencv::core::AlgorithmHint::ALGO_HINT_DEFAULT.into(),
    )?;

    // cornerMinEigenVal 角点检测
    let mut corner_response = Mat::default();
    imgproc::good_features_to_track(
        &gray,
        &mut corner_response,
        1000,
        0.01,
        10.,
        &core::no_array(),
        3,
        false,
        0.04,
    )?;

    // println!("{corner_response:?}");

    // 在原图上标记角点
    for i in 0..corner_response.rows() {
        let point = corner_response.at_row::<core::Point2f>(i)?;
        let (x, y) = (point[0].x as i32, point[0].y as i32);

        // 在角点位置画圆
        imgproc::circle(
            &mut img,
            core::Point::new(x, y),
            3,
            core::Scalar::new(0.0, 0.0, 255.0, 0.0),
            3,
            imgproc::LINE_8,
            0,
        )?;
    }

    highgui::named_window(&window_name, highgui::WINDOW_NORMAL)?;
    highgui::resize_window(&window_name, w, h)?;

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
