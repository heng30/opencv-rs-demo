use anyhow::Result;
use opencv::{core, core::Mat, highgui, imgcodecs, imgproc, prelude::*};

fn main() -> Result<()> {
    let (w, h) = (640, 480);
    let window_name = "img-watershed";

    let mut img = imgcodecs::imread("data/coins.png", imgcodecs::IMREAD_COLOR)?;

    let mut gray = Mat::default();
    imgproc::cvt_color(
        &img,
        &mut gray,
        imgproc::COLOR_BGR2GRAY,
        0,
        opencv::core::AlgorithmHint::ALGO_HINT_DEFAULT.into(),
    )?;

    // 二值化
    opencv::imgproc::threshold(
        &gray.clone(),
        &mut gray,
        0.,
        255.,
        opencv::imgproc::THRESH_BINARY_INV | opencv::imgproc::THRESH_OTSU,
    )?;

    // 降噪
    let kernel = opencv::imgproc::get_structuring_element(
        opencv::imgproc::MorphShapes::MORPH_RECT.into(),
        core::Size::new(3, 3),
        core::Point::new(-1, -1),
    )?;

    // 开运算
    opencv::imgproc::morphology_ex(
        &gray.clone(),
        &mut gray,
        opencv::imgproc::MORPH_OPEN,
        &kernel,
        core::Point::new(-1, -1),
        2,
        core::BORDER_CONSTANT,
        opencv::imgproc::morphology_default_border_value()?,
    )?;

    // 膨胀
    opencv::imgproc::dilate(
        &gray.clone(),
        &mut gray,
        &kernel,
        core::Point::new(-1, -1),
        1,
        core::BORDER_CONSTANT,
        opencv::imgproc::morphology_default_border_value()?,
    )?;

    // 获取前景物体
    let mut fg = Mat::default();

    // 根据距离重新计算白色快到其中心点的颜色，出来的效果是中间亮，四周逐渐变暗
    opencv::imgproc::distance_transform_def(&gray, &mut fg, opencv::imgproc::DIST_L2, 5)?;

    // 二值化
    opencv::imgproc::threshold(
        &fg.clone(),
        &mut fg,
        0.25 * 255.,
        255.,
        opencv::imgproc::THRESH_BINARY,
    )?;

    fg.clone().convert_to_def(&mut fg, core::CV_8UC1)?;

    // 暂时还不能确定是否是物体的区域
    let unknown = (&gray - &fg).into_result()?.to_mat()?;

    // 创建连通域, 用0标记背景，用1标记前景
    let mut marker = Mat::default();
    opencv::imgproc::connected_components_def(&fg, &mut marker)?;

    assert_eq!(marker.size()?, unknown.size()?);

    for i in 0..marker.rows() {
        for j in 0..marker.cols() {
            if *unknown.at_2d::<u8>(i, j)? == 255 {
                // 未知区域设置为0, watershed会使用
                *marker.at_2d_mut::<i32>(i, j)? = 0;
            } else {
                // 背景区域设置为1，前景区域大于1, 与unknown区域区分
                *marker.at_2d_mut::<i32>(i, j)? += 1;
            }
        }
    }

    // println!("{:?}", marker);

    // 确定未知区域，从而获得物体轮廓
    opencv::imgproc::watershed(&img, &mut marker)?;

    // 使用红色绘制图形边缘, 背景为1, 前景大于1, 边缘区域为-1
    for i in 0..marker.rows() {
        for j in 0..marker.cols() {
            // 边缘区域
            if *marker.at_2d::<i32>(i, j)? == -1 {
                *img.at_2d_mut::<core::Vec3b>(i, j)? = core::Vec3b::from_array([0, 0, 255]);
            }
        }
    }

    highgui::named_window(&window_name, highgui::WINDOW_NORMAL)?;
    highgui::resize_window(&window_name, w, h)?;
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
