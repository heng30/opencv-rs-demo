use anyhow::Result;
use opencv::{core, highgui, imgcodecs, imgproc, prelude::*};

fn main() -> Result<()> {
    let (w, h) = (640, 480);
    let window_name = "img-haar-car-number";

    let mut car_number_classifier =
        opencv::objdetect::CascadeClassifier::new("data/haarcascade_russian_plate_number.xml")?;

    let mut img = opencv::imgcodecs::imread("data/car.png", imgcodecs::IMREAD_COLOR)?;

    let mut gray = Mat::default();
    imgproc::cvt_color(
        &img,
        &mut gray,
        imgproc::COLOR_BGR2GRAY,
        0,
        opencv::core::AlgorithmHint::ALGO_HINT_DEFAULT.into(),
    )?;

    let mut car_number_objs = core::Vector::<core::Rect>::new();
    car_number_classifier.detect_multi_scale(
        &gray,
        &mut car_number_objs,
        1.1,
        5,
        0,
        core::Size::default(),
        core::Size::default(),
    )?;

    for rect in car_number_objs.into_iter() {
        // 绘制包围矩形
        opencv::imgproc::rectangle(
            &mut img,
            rect.clone(),
            opencv::core::Scalar::new(0., 0., 255., 0.),
            2,
            opencv::imgproc::LINE_AA,
            0,
        )?;

        // 获取识别区域
        let num_img = core::Mat::roi(&gray, rect)?;
        let mut num_img_bin = core::Mat::default();

        opencv::imgproc::threshold(
            &num_img,
            &mut num_img_bin,
            0.,
            255.,
            opencv::imgproc::THRESH_BINARY | opencv::imgproc::THRESH_OTSU,
        )?;

        // TODO: 进行ocr识别

        highgui::imshow(window_name, &num_img_bin)?;
    }

    highgui::named_window(window_name, highgui::WINDOW_NORMAL)?;
    highgui::resize_window(window_name, w, h)?;

    loop {
        let key = highgui::wait_key(0)?;
        if key & 0xFF == 'q' as i32 {
            break;
        }
    }

    highgui::destroy_all_windows()?;
    Ok(())
}
