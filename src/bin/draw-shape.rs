use anyhow::Result;
use opencv::highgui;
use opencv::prelude::*;
use std::sync::{
    atomic::{AtomicBool, AtomicI32, Ordering},
    Arc,
};

fn main() -> Result<()> {
    let (w, h) = (640, 480);
    let window_name = "draw-shape";

    let draw_flag = Arc::new(AtomicBool::new(false));
    let shape_ty = Arc::new(AtomicI32::new(0));
    let start_pos = Arc::new((AtomicI32::new(0), AtomicI32::new(0)));
    let end_pos = Arc::new((AtomicI32::new(0), AtomicI32::new(0)));

    let mut img = Mat::zeros(h, w, opencv::core::CV_8UC3)?.to_mat()?;

    highgui::named_window(window_name, highgui::WINDOW_NORMAL)?;
    highgui::resize_window(window_name, 640, 480)?;

    let draw_flag_ref = draw_flag.clone();
    let start_pos_ref = start_pos.clone();
    let end_pos_ref = end_pos.clone();

    highgui::set_mouse_callback(
        window_name,
        Some(Box::new(move |event, x, y, _flags| {
            // println!("event: {event} (x, y): ({x}, {y}) flags: {flags}");
            match event {
                1 /*EVENT_LBUTTONDOWN*/ => {
                    start_pos_ref.0.store(x, Ordering::Relaxed);
                    start_pos_ref.1.store(y, Ordering::Relaxed);
                }
                4 /* EVENT_LBUTTONUP */ => {
                    end_pos_ref.0.store(x, Ordering::Relaxed);
                    end_pos_ref.1.store(y, Ordering::Relaxed);
                    draw_flag_ref.store(true, Ordering::Relaxed);
                }
                _ => (),
            }
        })),
    )?;

    loop {
        // draw text
        let shape_type_str = match shape_ty.load(Ordering::Relaxed) {
            0 => "L",
            1 => "R",
            2 => "C",
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

        // draw shape
        if draw_flag.load(Ordering::Relaxed) {
            draw_flag.store(false, Ordering::Relaxed);

            let x = start_pos.0.load(Ordering::Relaxed);
            let y = start_pos.1.load(Ordering::Relaxed);
            let end_x = end_pos.0.load(Ordering::Relaxed);
            let end_y = end_pos.1.load(Ordering::Relaxed);
            let line_color = opencv::core::Scalar::new(0., 0., 255., 0.);
            let line_width = 2;

            match shape_ty.load(Ordering::Relaxed) {
                0 => {
                    opencv::imgproc::line(
                        &mut img,
                        opencv::core::Point::new(x, y),
                        opencv::core::Point::new(end_x, end_y),
                        line_color,
                        line_width,
                        opencv::imgproc::LINE_8,
                        0,
                    )?;
                }
                1 => {
                    let rec_w = (end_x - x).abs();
                    let rec_h = (end_y - y).abs();
                    let x = x.min(end_x);
                    let y = y.min(end_y);

                    opencv::imgproc::rectangle(
                        &mut img,
                        opencv::core::Rect::new(x, y, rec_w, rec_h),
                        line_color,
                        line_width,
                        opencv::imgproc::LINE_8,
                        0,
                    )?;
                }
                2 => {
                    let radius = (((end_x - x).pow(2) + (end_y - y).pow(2)) as f64).sqrt() as i32;
                    opencv::imgproc::circle(
                        &mut img,
                        opencv::core::Point::new(x, y),
                        radius,
                        line_color,
                        line_width,
                        opencv::imgproc::LINE_8,
                        0,
                    )?;
                }
                _ => unreachable!(),
            }
        }

        highgui::imshow(window_name, &img)?;

        let key = highgui::wait_key(40)?;
        if key & 0xFF == 'q' as i32 {
            break;
        } else if key & 0xFF == 'l' as i32 {
            shape_ty.store(0, Ordering::Relaxed);
        } else if key & 0xFF == 'r' as i32 {
            shape_ty.store(1, Ordering::Relaxed);
        } else if key & 0xFF == 'c' as i32 {
            shape_ty.store(2, Ordering::Relaxed);
        }
    }

    highgui::destroy_all_windows()?;
    Ok(())
}
