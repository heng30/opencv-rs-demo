use anyhow::Result;
use opencv::{core, core::Mat, highgui, imgcodecs, imgproc, prelude::*};

fn main() -> Result<()> {
    let (w, h) = (1280, 480);
    let (img_w, img_h) = (640, 480);
    let window_name = "img-joint";

    let mut img1 = imgcodecs::imread("data/city-left.png", imgcodecs::IMREAD_COLOR)?;
    let mut img2 = imgcodecs::imread("data/city-right.png", imgcodecs::IMREAD_COLOR)?;

    opencv::imgproc::resize(
        &img1.clone(),
        &mut img1,
        core::Size::new(img_w, img_h),
        0.,
        0.,
        opencv::imgproc::INTER_AREA,
    )?;

    opencv::imgproc::resize(
        &img2.clone(),
        &mut img2,
        core::Size::new(img_w, img_h),
        0.,
        0.,
        opencv::imgproc::INTER_AREA,
    )?;

    // 图片水平拼接
    // let mut img = core::Mat::default();
    // core::hconcat2(&img2, &img1, &mut img)?;

    // 获得单硬性矩阵
    let homography = get_homography(&img1, &img2)?;

    // println!("{:?}", homography);

    // 进行图片拼接
    let img = stitch_img(&img1, &img2, &homography)?;

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

fn get_homography(img1: &Mat, img2: &Mat) -> Result<Mat> {
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
    let mut bf = opencv::features2d::BFMatcher::create(core::NormTypes::NORM_L1.into(), false)?;
    let mut matches = core::Vector::<core::Vector<core::DMatch>>::new();

    // 进行匹配
    bf.add(&dps2)?;
    bf.knn_match_def(&dps1, &mut matches, 2)?;

    let mut good_matches = core::Vector::<core::Vector<core::DMatch>>::new();

    // 过滤特征点
    for item in matches.into_iter() {
        if item.len() == 1 {
            good_matches.push(item);
            continue;
        }

        let distance1 = item.get(0).unwrap().distance;
        let distance2 = item.get(1).unwrap().distance;
        // println!("{distance1}, {distance2}");

        // 算法有问题
        if distance1 < distance2 * 0.7 {
            good_matches.push(item);
        }
    }

    if good_matches.len() >= 8 {
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

        return Ok(homography);
    } else {
        unreachable!("good_matches length than 4")
    }
}

fn stitch_img(img1: &Mat, img2: &Mat, homography: &Mat) -> Result<Mat> {
    // 获取图的4个角点
    let (h1, w1) = (img1.rows(), img1.cols());
    let mut img1_dims = Mat::from_slice_2d(&[[0, 0], [0, h1 - 1], [w1 - 1, h1 - 1], [w1 - 1, 0]])?;
    img1_dims
        .clone()
        .convert_to_def(&mut img1_dims, core::CV_32FC2)?;
    let img1_dims = img1_dims.reshape(2, 4)?;

    let (h2, w2) = (img2.rows(), img2.cols());
    let mut img2_dims = Mat::from_slice_2d(&[[0, 0], [0, h2 - 1], [w2 - 1, h1 - 1], [w2 - 1, 0]])?;
    img2_dims
        .clone()
        .convert_to_def(&mut img2_dims, core::CV_32FC2)?;
    let img2_dims = img2_dims.reshape(2, 4)?;

    //获取变换后的坐标
    let mut img1_transform = Mat::default();
    core::perspective_transform(&img1_dims, &mut img1_transform, &homography)?;

    // 计算合成图像的大小
    let mut img_dims = vec![];
    for i in 0..img2_dims.rows() {
        let p = img2_dims.at_2d::<core::Point2f>(i, 0)?;
        img_dims.push(p);
    }

    for i in 0..img1_transform.rows() {
        let p = img1_transform.at_2d::<core::Point2f>(i, 0)?;
        img_dims.push(p);
    }

    // println!("{:?}", img_dims);

    let (mut min_x, mut min_y, mut max_x, mut max_y) = (i32::MAX, i32::MAX, i32::MIN, i32::MIN);
    for p in img_dims.iter() {
        let px = p.x as i32;
        let py = p.y as i32;

        if min_x > px {
            min_x = px;
        }

        if max_x < px {
            max_x = px;
        }

        if min_y > py {
            min_y = py;
        }

        if max_y < py {
            max_y = py;
        }
    }

    // println!("{min_x}, {min_y}, {max_x}, {max_y}");

    // 创建平移矩阵
    let transform_mat = Mat::from_slice_2d(&[
        [1., 0., -min_x as f64],
        [0., 1., -min_y as f64],
        [0., 0., 1.],
    ])?;

    // 复合矩阵
    let transform_mat = (transform_mat * homography).into_result()?;

    // println!("{:?}", transform_mat);

    // 绘制变换后的img1
    let mut img = Mat::default();
    opencv::imgproc::warp_perspective_def(
        &img1,
        &mut img,
        &transform_mat,
        core::Size::new(max_x - min_x, max_y - min_y),
    )?;

    // 绘制变换后的img2
    let roi_width = w2.min(max_x);
    let roi_height = h2.min(max_y);

    let img_rect = core::Rect::new(-min_x, -min_y, roi_width, roi_height);
    let img2_rect = core::Rect::new(0, 0, roi_width, roi_height);

    // println!("{:?}", img_rect);
    // println!("{:?}", img2_rect);

    let mut img_roi = core::Mat::roi_mut(&mut img, img_rect)?;
    let img2_roi = core::Mat::roi(img2, img2_rect)?;
    img2_roi.copy_to(&mut img_roi)?;

    Ok(img)
}
