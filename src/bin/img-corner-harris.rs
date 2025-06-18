use anyhow::Result;
use opencv::{core, core::Mat, highgui, imgcodecs, imgproc, prelude::*};

fn main() -> Result<()> {
    let (w, h) = (640, 480);
    let window_name = "img-corner-harris";

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

    // Harris 角点检测
    let mut corner_response = Mat::default();
    imgproc::corner_harris(
        &gray,
        &mut corner_response,
        3,
        7,
        0.03,
        core::BORDER_DEFAULT,
    )?;

    let kernel = opencv::imgproc::get_structuring_element(
        opencv::imgproc::MorphShapes::MORPH_CROSS.into(),
        core::Size::new(3, 3),
        core::Point::new(-1, -1),
    )?;

    opencv::imgproc::erode(
        &corner_response.clone(),
        &mut corner_response,
        &kernel,
        core::Point::new(-1, -1),
        1,
        core::BORDER_CONSTANT,
        opencv::imgproc::morphology_default_border_value()?,
    )?;

    // highgui::imshow(window_name, &corner_response)?;

    // 在原图上标记角点
    let threshold = 3.0; // 阈值可以根据实际情况调整
    for i in 0..corner_response.rows() {
        for j in 0..corner_response.cols() {
            let response = *corner_response.at_2d::<f32>(i, j)?;
            if response > threshold {
                // 在角点位置画圆
                imgproc::circle(
                    &mut img,
                    core::Point::new(j, i),
                    3,
                    core::Scalar::new(0.0, 0.0, 255.0, 0.0),
                    2,
                    imgproc::LINE_8,
                    0,
                )?;
            }
        }
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
