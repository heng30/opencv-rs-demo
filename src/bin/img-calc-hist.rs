use anyhow::Result;
use opencv::{
    core,
    core::{Scalar, Vector},
    highgui, imgcodecs, imgproc,
    prelude::*,
};

fn main() -> Result<()> {
    let (w, h) = (640, 480);
    let img = imgcodecs::imread("data/lena.png", imgcodecs::IMREAD_COLOR)?;
    let window_name = "img-calc-hist";

    let imgs = Vector::<Mat>::from(vec![img.clone()]);

    // 计算 BGR 三通道直方图
    let hist_size = Vector::from_slice(&[256]); // 256 bins
    let ranges = Vector::from_slice(&[0.0, 256.0]); // 像素范围 0~255

    let mut hist_b = Mat::default();
    let mut hist_g = Mat::default();
    let mut hist_r = Mat::default();

    // 计算 B 通道直方图
    imgproc::calc_hist(
        &imgs,
        &Vector::from(vec![0]), // B 通道
        &core::no_array(),
        &mut hist_b,
        &hist_size,
        &ranges,
        false, // 不累加
    )?;

    // 计算 G 通道直方图
    imgproc::calc_hist(
        &imgs,
        &Vector::from(vec![1]), // B 通道
        &core::no_array(),
        &mut hist_g,
        &hist_size,
        &ranges,
        false,
    )?;

    // 计算 R 通道直方图
    imgproc::calc_hist(
        &imgs,
        &Vector::from(vec![2]), // B 通道
        &core::no_array(),
        &mut hist_r,
        &hist_size,
        &ranges,
        false,
    )?;

    // 创建直方图画布
    let hist_w = 512;
    let hist_h = 400;
    let bin_w = (hist_w as f64 / 256.0) as i32;
    let mut hist_image =
        Mat::new_rows_cols_with_default(hist_h, hist_w, core::CV_8UC3, Scalar::all(0.0))?;

    // 归一化直方图数据（使其适应画布高度）
    let mut hist_b_norm = Mat::default();
    let mut hist_g_norm = Mat::default();
    let mut hist_r_norm = Mat::default();

    core::normalize(
        &hist_b,
        &mut hist_b_norm,
        0.0,
        hist_h as f64,
        core::NORM_MINMAX,
        -1,
        &core::no_array(),
    )?;
    core::normalize(
        &hist_g,
        &mut hist_g_norm,
        0.0,
        hist_h as f64,
        core::NORM_MINMAX,
        -1,
        &core::no_array(),
    )?;
    core::normalize(
        &hist_r,
        &mut hist_r_norm,
        0.0,
        hist_h as f64,
        core::NORM_MINMAX,
        -1,
        &core::no_array(),
    )?;

    // 绘制 BGR 三通道直方图
    for i in 1..256 {
        // 绘制 B 通道（蓝色）
        imgproc::line(
            &mut hist_image,
            core::Point::new(
                bin_w * (i - 1),
                hist_h - (*hist_b_norm.at_2d::<f32>(i - 1, 0)? as i32),
            ),
            core::Point::new(
                bin_w * i,
                hist_h - (*hist_b_norm.at_2d::<f32>(i, 0)? as i32),
            ),
            Scalar::new(255.0, 0.0, 0.0, 0.0), // 蓝色
            2,
            8,
            0,
        )?;

        // 绘制 G 通道（绿色）
        imgproc::line(
            &mut hist_image,
            core::Point::new(
                bin_w * (i - 1),
                hist_h - (*hist_g_norm.at_2d::<f32>(i - 1, 0)? as i32),
            ),
            core::Point::new(
                bin_w * i,
                hist_h - (*hist_g_norm.at_2d::<f32>(i, 0)? as i32),
            ),
            Scalar::new(0.0, 255.0, 0.0, 0.0), // 绿色
            2,
            8,
            0,
        )?;

        // 绘制 R 通道（红色）
        imgproc::line(
            &mut hist_image,
            core::Point::new(
                bin_w * (i - 1),
                hist_h - (*hist_r_norm.at_2d::<f32>(i - 1, 0)? as i32),
            ),
            core::Point::new(
                bin_w * i,
                hist_h - (*hist_r_norm.at_2d::<f32>(i, 0)? as i32),
            ),
            Scalar::new(0.0, 0.0, 255.0, 0.0), // 红色
            2,
            8,
            0,
        )?;
    }

    // 显示直方图
    highgui::named_window(window_name, highgui::WINDOW_NORMAL)?;
    highgui::resize_window(window_name, w, h)?;
    highgui::imshow(window_name, &hist_image)?;

    loop {
        let key = highgui::wait_key(0)?;
        if key & 0xFF == 'q' as i32 {
            break;
        }
    }

    highgui::destroy_all_windows()?;
    Ok(())
}
