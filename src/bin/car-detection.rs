use anyhow::Result;
use opencv::{core, core::Mat, highgui, imgproc, prelude::*, videoio::VideoCapture};

fn main() -> Result<()> {
    let window_name = "car-statistic";

    let mut cap = VideoCapture::from_file("data/running-cars.mp4", opencv::videoio::CAP_ANY)?;
    let fps = cap.get(opencv::videoio::CAP_PROP_FPS)?;
    let delay = (1000.0 / fps) as i32; // ms per frame
    let width = cap.get(opencv::videoio::CAP_PROP_FRAME_WIDTH)? as i32;
    let height = cap.get(opencv::videoio::CAP_PROP_FRAME_HEIGHT)? as i32;

    println!("video size: ({width}, {height})");

    highgui::named_window(&window_name, highgui::WINDOW_NORMAL)?;
    highgui::resize_window(&window_name, width, height)?;

    // 腐蚀/膨胀核
    let kernel = opencv::imgproc::get_structuring_element(
        opencv::imgproc::MorphShapes::MORPH_RECT.into(),
        core::Size::new(3, 3),
        core::Point::new(-1, -1),
    )?;

    // 高斯模糊去背景
    let mut bg_sub_mog = opencv::bgsegm::create_background_subtractor_mog(200, 5, 0.7, 0.)?;

    // 最小的汽车大小，小于此大小，不绘制包围矩形
    let (min_car_width, min_car_height) = (40, 40);

    // 检测线的y轴位置
    let test_line_y = height / 3 * 2;

    // 检测线x个像素内的才绘制包围矩形
    let bounding_box_test_line_range = height / 5;

    // 统计车辆数据
    let mut car_counts = 0;

    loop {
        let start = std::time::Instant::now();

        let mut frame = Mat::default();
        if let Ok(true) = cap.read(&mut frame) {
            let mut src_frame = frame.clone();

            // 灰度
            imgproc::cvt_color(&frame.clone(), &mut frame, imgproc::COLOR_BGR2GRAY, 0)?;

            // 高斯降噪
            opencv::imgproc::gaussian_blur(
                &frame.clone(),
                &mut frame,
                core::Size::new(3, 3),
                5.,
                5.,
                core::BORDER_DEFAULT,
                // core::ALGO_HINT_DEFAULT,
            )?;

            // 去背景
            bg_sub_mog.apply(&frame.clone(), &mut frame, -1.)?;

            // 腐蚀, 去除背景噪点
            opencv::imgproc::erode(
                &frame.clone(),
                &mut frame,
                &kernel,
                core::Point::new(-1, -1),
                1,
                core::BORDER_CONSTANT,
                opencv::imgproc::morphology_default_border_value()?,
            )?;

            // 膨胀，还原腐蚀减少的物体轮廓
            opencv::imgproc::dilate(
                &frame.clone(),
                &mut frame,
                &kernel,
                core::Point::new(-1, -1),
                5,
                core::BORDER_CONSTANT,
                opencv::imgproc::morphology_default_border_value()?,
            )?;

            // 闭操作，消除物体内部的黑色噪点
            for _ in 0..3 {
                opencv::imgproc::morphology_ex(
                    &frame.clone(),
                    &mut frame,
                    opencv::imgproc::MORPH_CLOSE,
                    &kernel,
                    core::Point::new(-1, -1),
                    5,
                    core::BORDER_CONSTANT,
                    opencv::imgproc::morphology_default_border_value()?,
                )?;
            }

            // 绘制检测线，在线附近的测量会被统计
            opencv::imgproc::line(
                &mut src_frame,
                opencv::core::Point::new(0, test_line_y),
                opencv::core::Point::new(width, test_line_y),
                opencv::core::Scalar::new(255., 0., 0., 0.),
                2,
                opencv::imgproc::LINE_8,
                0,
            )?;

            // 查找轮廓
            let mut contours = core::Vector::<core::Mat>::new();
            imgproc::find_contours(
                &frame,
                &mut contours,
                imgproc::RETR_TREE,
                imgproc::CHAIN_APPROX_SIMPLE,
                core::Point::default(),
            )?;

            // 在原图上绘制所有包围矩形
            for contour in contours.into_iter() {
                let bounding_rect = opencv::imgproc::bounding_rect(&contour)?;

                if bounding_rect.y < test_line_y - bounding_box_test_line_range {
                    continue;
                }

                if bounding_rect.width < min_car_width && bounding_rect.height < min_car_height {
                    continue;
                }

                // 统计车辆数据
                let offset = 3;
                let bouding_rect_center_y = bounding_rect.y + bounding_rect.height / 2;
                if bouding_rect_center_y > test_line_y - offset
                    && bouding_rect_center_y < test_line_y + offset
                {
                    car_counts += 1;
                }

                // println!("car counts: {car_counts}");

                // 绘制包围进行
                opencv::imgproc::rectangle(
                    &mut src_frame,
                    bounding_rect,
                    opencv::core::Scalar::new(0., 0., 255., 0.),
                    2,
                    opencv::imgproc::LINE_4,
                    0,
                )?;
            }

            // 绘制统计数据
            opencv::imgproc::put_text(
                &mut src_frame,
                &format!("Vehicle:{car_counts}"),
                opencv::core::Point::new(10, 40),
                opencv::imgproc::HersheyFonts::FONT_HERSHEY_DUPLEX.into(),
                1.,
                opencv::core::Scalar::new(0., 0., 255., 0.),
                1,
                opencv::imgproc::LINE_4,
                false,
            )?;

            // highgui::imshow(window_name, &frame)?;
            highgui::imshow(window_name, &src_frame)?;
        }

        // Calculate processing time and adjust wait time
        let elapsed = start.elapsed().as_millis() as i32;
        let wait_time = std::cmp::max(1, delay - elapsed);

        let key = highgui::wait_key(wait_time)?;
        if key & 0xFF == 'q' as i32 {
            break;
        }
    }

    cap.release()?;
    highgui::destroy_all_windows()?;

    Ok(())
}
