use anyhow::Result;
use opencv::{core, core::Vector, highgui, imgcodecs, imgproc, prelude::*};

fn main() -> Result<()> {
    let (w, h) = (640, 480);
    let window_name = "img-meanshift";
    let mut img = imgcodecs::imread("data/flower.png", imgcodecs::IMREAD_COLOR)?;
    let mut img_orig = img.clone();

    highgui::named_window(window_name, highgui::WINDOW_NORMAL)?;
    highgui::resize_window(window_name, w, h)?;

    // 相近色彩合并
    opencv::imgproc::pyr_mean_shift_filtering_def(&img.clone(), &mut img, 20., 30.)?;

    // 生成图像轮廓
    opencv::imgproc::canny(&img.clone(), &mut img, 150., 300., 3, false)?;

    // 查找轮廓
    let mut contours = Vector::<Mat>::new();
    imgproc::find_contours(
        &img,
        &mut contours,
        imgproc::RETR_TREE,
        imgproc::CHAIN_APPROX_SIMPLE,
        core::Point::default(),
    )?;

    // 在原图上绘制轮廓
    imgproc::draw_contours(
        &mut img_orig,
        &contours,
        -1, // draw all contours
        core::Scalar::new(0., 0., 255., 0.),
        2, // thickness
        imgproc::LINE_8,
        &core::no_array(),
        2, // maxLevel
        core::Point::default(),
    )?;

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
