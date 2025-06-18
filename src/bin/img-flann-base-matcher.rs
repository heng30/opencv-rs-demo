use anyhow::Result;
use opencv::{core, core::Mat, highgui, imgcodecs, imgproc, prelude::*};

fn main() -> Result<()> {
    let (w, h) = (640, 480);
    let window_name = "img-flann-base-matcher";

    let img1 = imgcodecs::imread("data/opencv1.png", imgcodecs::IMREAD_COLOR)?;
    let img2 = imgcodecs::imread("data/opencv2.png", imgcodecs::IMREAD_COLOR)?;

    // 灰度
    let mut gray1 = Mat::default();
    imgproc::cvt_color(
        &img1,
        &mut gray1,
        imgproc::COLOR_BGR2GRAY,
        0,
        opencv::core::AlgorithmHint::ALGO_HINT_DEFAULT.into(),
    )?;

    let mut gray2 = Mat::default();
    imgproc::cvt_color(
        &img2,
        &mut gray2,
        imgproc::COLOR_BGR2GRAY,
        0,
        opencv::core::AlgorithmHint::ALGO_HINT_DEFAULT.into(),
    )?;

    // 获取特征描述
    let mut sift = opencv::features2d::SIFT::create_def()?;
    let mut kps1 = core::Vector::<core::KeyPoint>::new();
    let mut dps1 = core::Mat::default();
    sift.detect_and_compute_def(&gray1, &core::no_array(), &mut kps1, &mut dps1)?;

    let mut kps2 = core::Vector::<core::KeyPoint>::new();
    let mut dps2 = core::Mat::default();
    sift.detect_and_compute_def(&gray2, &core::no_array(), &mut kps2, &mut dps2)?;

    // 创建匹配器
    let mut flann = opencv::features2d::FlannBasedMatcher::new(
        &core::Ptr::new(opencv::flann::IndexParams::from({
            let mut p = opencv::flann::KDTreeIndexParams::new(5)?;
            p.set_algorithm(1)?;
            p
        })),
        &core::Ptr::new(opencv::flann::SearchParams::new_1(50, 0., true)?),
    )?;

    let mut matches = core::Vector::<core::Vector<core::DMatch>>::new();

    opencv::prelude::FlannBasedMatcherTrait::add(&mut flann, &dps2)?;
    flann.knn_match_def(&dps1, &mut matches, 2)?;

    let mut good_matches = core::Vector::<core::Vector<core::DMatch>>::new();

    for item in matches.into_iter() {
        if item.len() == 1 {
            good_matches.push(item);
            continue;
        }

        let distance1 = item.get(0).unwrap().distance;
        let distance2 = item.get(1).unwrap().distance;
        println!("{distance1}, {distance2}");

        if distance1 < distance2 * 0.7 {
            good_matches.push(core::Vector::from_slice(&[item.get(0).unwrap()]));
        }
    }

    // 绘制匹配关系
    let mut img = Mat::default();
    opencv::features2d::draw_matches_knn_def(&img1, &kps1, &img2, &kps2, &good_matches, &mut img)?;

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
