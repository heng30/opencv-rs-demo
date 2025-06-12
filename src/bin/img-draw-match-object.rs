use anyhow::Result;
use opencv::{core, core::Mat, highgui, imgcodecs, imgproc, prelude::*};

fn main() -> Result<()> {
    let (w, h) = (640, 480);
    let window_name = "img-draw-match-object";

    let img1 = imgcodecs::imread("data/opencv1.png", imgcodecs::IMREAD_COLOR)?;
    let mut img2 = imgcodecs::imread("data/opencv2.png", imgcodecs::IMREAD_COLOR)?;

    // 灰度
    let mut gray1 = Mat::default();
    imgproc::cvt_color(&img1, &mut gray1, imgproc::COLOR_BGR2GRAY, 0)?;

    let mut gray2 = Mat::default();
    imgproc::cvt_color(&img2, &mut gray2, imgproc::COLOR_BGR2GRAY, 0)?;

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

    // 进行匹配
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
            good_matches.push(core::Vector::from_slice(&vec![item.get(0).unwrap()]));
        }
    }

    if good_matches.len() >= 4 {
        // 获取homography参数点列表
        let mut src_pts = vec![];
        for item in good_matches.iter() {
            for item2 in item.iter() {
                src_pts.push(kps1.get(item2.query_idx as usize).unwrap().pt());
            }
        }
        let src_pts = Mat::from_slice(&src_pts)?;

        let mut dst_pts = vec![];
        for item in good_matches.iter() {
            for item2 in item.iter() {
                dst_pts.push(kps2.get(item2.train_idx as usize).unwrap().pt());
            }
        }
        let dst_pts = Mat::from_slice(&dst_pts)?;

        let homography = opencv::calib3d::find_homography(
            &src_pts,
            &dst_pts,
            &mut core::Mat::default(),
            opencv::calib3d::RANSAC,
            5.0,
        )?;
        // println!("{:?}", homography);

        let img1_width = img1.cols() as f32;
        let img1_height = img1.rows() as f32;
        let img1_corner_pts = Mat::from_slice_2d(&[
            [0.0_f32, 0.0_f32],
            [0.0_f32, img1_height - 1.],
            [img1_width - 1., img1_height - 1.],
            [img1_width - 1., 0.0_f32],
        ])?;

        let img1_corner_pts = img1_corner_pts.reshape(2, 4)?;

        // println!("{:?}", img1_corner_pts);

        let mut pt_dst = Mat::default();
        core::perspective_transform(&img1_corner_pts, &mut pt_dst, &homography)?;
        pt_dst.clone().convert_to_def(&mut pt_dst, core::CV_32SC2)?;

        // println!("{:?}", pt_dst);

        // 绘制包围框
        opencv::imgproc::polylines(
            &mut img2,
            &pt_dst,
            true,
            opencv::core::Scalar::new(0., 255., 255., 0.),
            1,
            opencv::imgproc::LINE_AA,
            0,
        )?;
    } else {
        unreachable!("good_matches length than 4")
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
