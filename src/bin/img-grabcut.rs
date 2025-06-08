use anyhow::Result;
use opencv::{core, highgui, imgcodecs, prelude::*};
use std::sync::{
    atomic::{AtomicI32, Ordering},
    Arc,
};

fn main() -> Result<()> {
    let window_name = "img-grabcut";
    let mut img = imgcodecs::imread("data/lena.png", imgcodecs::IMREAD_COLOR)?;
    let img_orig = img.clone();
    let mut img_output = Mat::zeros(img.rows(), img.cols(), img.typ())?.to_mat()?;

    highgui::named_window(window_name, highgui::WINDOW_AUTOSIZE)?;

    // let (w, h) = (640, 480);
    // highgui::named_window(window_name, highgui::WINDOW_NORMAL)?;
    // highgui::resize_window(window_name, w, h)?;

    let draw_flag = Arc::new(AtomicI32::new(0));
    let shape_ty = Arc::new(AtomicI32::new(0));
    let start_pos = Arc::new((AtomicI32::new(0), AtomicI32::new(0)));
    let end_pos = Arc::new((AtomicI32::new(0), AtomicI32::new(0)));

    let draw_flag_ref = draw_flag.clone();
    let start_pos_ref = start_pos.clone();
    let end_pos_ref = end_pos.clone();

    // 处理鼠标事件
    highgui::set_mouse_callback(
        window_name,
        Some(Box::new(move |event, x, y, _flags| {
            let event = highgui::MouseEventTypes::try_from(event).unwrap();
            match event {
                highgui::MouseEventTypes::EVENT_LBUTTONDOWN => {
                    start_pos_ref.0.store(x, Ordering::Relaxed);
                    start_pos_ref.1.store(y, Ordering::Relaxed);
                    end_pos_ref.0.store(x, Ordering::Relaxed);
                    end_pos_ref.1.store(y, Ordering::Relaxed);
                    draw_flag_ref.store(1, Ordering::Relaxed);
                }
                highgui::MouseEventTypes::EVENT_LBUTTONUP => {
                    end_pos_ref.0.store(x, Ordering::Relaxed);
                    end_pos_ref.1.store(y, Ordering::Relaxed);
                    draw_flag_ref.store(2, Ordering::Relaxed);
                }
                highgui::MouseEventTypes::EVENT_MOUSEMOVE => {
                    let draw_flag = draw_flag_ref.load(Ordering::Relaxed);
                    if draw_flag == 1 {
                        draw_flag_ref.store(3, Ordering::Relaxed);
                    } else if draw_flag == 3 {
                        end_pos_ref.0.store(x, Ordering::Relaxed);
                        end_pos_ref.1.store(y, Ordering::Relaxed);
                    }
                }
                _ => (),
            }
        })),
    )?;

    loop {
        // 绘制按键字符
        let shape_type_str = match shape_ty.load(Ordering::Relaxed) {
            0 => "R",
            _ => unreachable!(),
        };

        opencv::imgproc::rectangle(
            &mut img,
            opencv::core::Rect::new(30, 10, 20, 20),
            opencv::core::Scalar::all(0.),
            -1,
            opencv::imgproc::LINE_4,
            0,
        )?;

        opencv::imgproc::put_text(
            &mut img,
            shape_type_str,
            opencv::core::Point::new(30, 30),
            opencv::imgproc::HersheyFonts::FONT_HERSHEY_PLAIN.into(),
            2.,
            opencv::core::Scalar::new(0., 255., 255., 0.),
            1,
            opencv::imgproc::LINE_4,
            false,
        )?;

        // 绘制图形
        let x = start_pos.0.load(Ordering::Relaxed);
        let y = start_pos.1.load(Ordering::Relaxed);
        let end_x = end_pos.0.load(Ordering::Relaxed);
        let end_y = end_pos.1.load(Ordering::Relaxed);
        let draw_flag_ref = draw_flag.load(Ordering::Relaxed);
        let red_line = opencv::core::Scalar::new(0., 0., 255., 0.);
        let blue_line = opencv::core::Scalar::new(255., 0., 0., 0.);

        match shape_ty.load(Ordering::Relaxed) {
            // 矩形
            0 => {
                let rec_w = (end_x - x).abs();
                let rec_h = (end_y - y).abs();
                let x = x.min(end_x);
                let y = y.min(end_y);
                let line_width = 2;

                if draw_flag_ref == 2 {
                    // 绘制最终的矩形
                    opencv::imgproc::rectangle(
                        &mut img,
                        opencv::core::Rect::new(x, y, rec_w, rec_h),
                        red_line,
                        line_width,
                        opencv::imgproc::LINE_8,
                        0,
                    )?;

                    draw_flag.store(0, Ordering::Relaxed);
                } else if draw_flag_ref == 3 {
                    // 绘制移动中的矩形
                    let mut img_copy = img.clone();
                    opencv::imgproc::rectangle(
                        &mut img_copy,
                        opencv::core::Rect::new(x, y, rec_w, rec_h),
                        blue_line,
                        line_width,
                        opencv::imgproc::LINE_8,
                        0,
                    )?;

                    highgui::imshow(window_name, &img_copy)?;
                }
            }
            _ => unreachable!(),
        }

        // 原图
        if draw_flag_ref != 3 {
            highgui::imshow(window_name, &img)?;
        }

        // 抠图
        highgui::imshow(&format!("{window_name}-output"), &img_output)?;

        let key = highgui::wait_key(20)?;
        if key & 0xFF == 'q' as i32 {
            break;
        } else if key & 0xFF == 'r' as i32 {
            shape_ty.store(0, Ordering::Relaxed);
        } else if key & 0xFF == 'g' as i32 {
            // 进行抠图

            // mask值含义. 背景：0, 前景：1, 可能是背景：2, 可能是前景：3
            let mut mask = Mat::zeros(img.rows(), img.cols(), core::CV_8UC1)?.to_mat()?;
            let mut bgdmodel = Mat::zeros(1, 65, core::CV_64FC1)?.to_mat()?;
            let mut fgdmodel = bgdmodel.clone();

            // 获取矩形
            let x = start_pos.0.load(Ordering::Relaxed);
            let y = start_pos.1.load(Ordering::Relaxed);
            let end_x = end_pos.0.load(Ordering::Relaxed);
            let end_y = end_pos.1.load(Ordering::Relaxed);
            let rec_w = (end_x - x).abs();
            let rec_h = (end_y - y).abs();
            let x = x.min(end_x);
            let y = y.min(end_y);

            let bounding_rect = opencv::core::Rect::new(x, y, rec_w, rec_h);

            if bounding_rect.width > 0 && bounding_rect.height > 0 {
                opencv::imgproc::grab_cut(
                    &img_orig,
                    &mut mask,
                    bounding_rect,
                    &mut bgdmodel,
                    &mut fgdmodel,
                    1,
                    opencv::imgproc::GrabCutModes::GC_INIT_WITH_RECT.into(),
                )?;

                // 前景色设为255,背景色设为0
                for i in 0..mask.rows() {
                    for j in 0..mask.cols() {
                        let p = *mask.at_2d::<u8>(i, j)?;
                        if p == 1 || p == 3 {
                            *mask.at_2d_mut::<u8>(i, j)? = 255;
                        } else {
                            *mask.at_2d_mut::<u8>(i, j)? = 0;
                        }
                    }
                }

                core::bitwise_and(&img_orig, &img_orig, &mut img_output, &mask)?;
            }
        }
    }

    highgui::destroy_all_windows()?;
    Ok(())
}
