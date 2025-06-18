use anyhow::Result;
use opencv::{core, highgui, imgcodecs, imgproc, prelude::*};

fn main() -> Result<()> {
    let (w, h) = (640, 480);
    let window_name = "img-haar";

    // 仅对正脸的识别正确率高
    let mut face_classifier =
        opencv::objdetect::CascadeClassifier::new("data/haarcascade_frontalface_default.xml")?;

    let mut img = opencv::imgcodecs::imread("data/p1.png", imgcodecs::IMREAD_COLOR)?;

    let mut gray = Mat::default();
    imgproc::cvt_color(
        &img,
        &mut gray,
        imgproc::COLOR_BGR2GRAY,
        0,
        opencv::core::AlgorithmHint::ALGO_HINT_DEFAULT.into(),
    )?;

    let mut objs = core::Vector::<core::Rect>::new();
    face_classifier.detect_multi_scale(
        &gray,
        &mut objs,
        1.1,
        5,
        0,
        core::Size::default(),
        core::Size::default(),
    )?;

    println!("{:?}", objs);

    for rect in objs.into_iter() {
        opencv::imgproc::rectangle(
            &mut img,
            rect,
            opencv::core::Scalar::new(0., 0., 255., 0.),
            2,
            opencv::imgproc::LINE_AA,
            0,
        )?;
    }

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
