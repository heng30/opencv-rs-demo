use anyhow::Result;
use opencv::{core, highgui, imgcodecs, imgproc, prelude::*};

fn main() -> Result<()> {
    let (w, h) = (640, 480);
    let window_name = "img-haar-all";

    let mut face_classifier =
        opencv::objdetect::CascadeClassifier::new("data/haarcascade_frontalface_default.xml")?;

    let mut eye_classifier = opencv::objdetect::CascadeClassifier::new("data/haarcascade_eye.xml")?;

    // let mut mouth_classifier =
    //     opencv::objdetect::CascadeClassifier::new("data/haarcascade_mcs_mouth.xml")?;
    //
    // let mut nose_classifier =
    //     opencv::objdetect::CascadeClassifier::new("data/haarcascade_mcs_nose.xml")?;

    let mut img = opencv::imgcodecs::imread("data/p1.png", imgcodecs::IMREAD_COLOR)?;

    let mut gray = Mat::default();
    imgproc::cvt_color(&img, &mut gray, imgproc::COLOR_BGR2GRAY, 0)?;

    let mut face_objs = core::Vector::<core::Rect>::new();
    face_classifier.detect_multi_scale(
        &gray,
        &mut face_objs,
        1.1,
        5,
        0,
        core::Size::default(),
        core::Size::default(),
    )?;

    for rect in face_objs.into_iter() {
        opencv::imgproc::rectangle(
            &mut img,
            rect.clone(),
            opencv::core::Scalar::new(0., 0., 255., 0.),
            2,
            opencv::imgproc::LINE_AA,
            0,
        )?;

        // 获取脸部区域
        let mut img_roi = core::Mat::roi_mut(&mut img, rect)?;
        let gray_roi = core::Mat::roi(&gray, rect)?;

        // 对眼睛进行识别
        let mut eye_objs = core::Vector::<core::Rect>::new();
        eye_classifier.detect_multi_scale(
            &gray_roi,
            &mut eye_objs,
            1.1,
            3,
            0,
            core::Size::default(),
            core::Size::default(),
        )?;

        for rect in eye_objs.into_iter() {
            opencv::imgproc::rectangle(
                &mut img_roi,
                rect,
                opencv::core::Scalar::new(255., 0., 0., 0.),
                2,
                opencv::imgproc::LINE_AA,
                0,
            )?;
        }

        // 对嘴进行识别, 效果不好
        // let mut mouth_objs = core::Vector::<core::Rect>::new();
        // mouth_classifier.detect_multi_scale(
        //     &gray_roi,
        //     &mut mouth_objs,
        //     1.1,
        //     3,
        //     0,
        //     core::Size::default(),
        //     core::Size::default(),
        // )?;
        //
        // for rect in mouth_objs.into_iter() {
        //     opencv::imgproc::rectangle(
        //         &mut img_roi,
        //         rect,
        //         opencv::core::Scalar::new(0., 255., 0., 0.),
        //         2,
        //         opencv::imgproc::LINE_AA,
        //         0,
        //     )?;
        // }

        // 对鼻子进行识别, 效果不好
        // let mut nose_objs = core::Vector::<core::Rect>::new();
        // nose_classifier.detect_multi_scale(
        //     &gray_roi,
        //     &mut nose_objs,
        //     1.1,
        //     3,
        //     0,
        //     core::Size::default(),
        //     core::Size::default(),
        // )?;
        //
        // for rect in nose_objs.into_iter() {
        //     opencv::imgproc::rectangle(
        //         &mut img_roi,
        //         rect,
        //         opencv::core::Scalar::new(255., 255., 0., 0.),
        //         2,
        //         opencv::imgproc::LINE_AA,
        //         0,
        //     )?;
        // }
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
