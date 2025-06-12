use anyhow::Result;
use opencv::{core, highgui, imgcodecs, prelude::*};

fn main() -> Result<()> {
    let (w, h) = (640, 480);
    let window_name = "img-match-template";
    let mut img = imgcodecs::imread("data/lena.png", imgcodecs::IMREAD_COLOR)?;
    let img_template = imgcodecs::imread("data/lena-face.png", imgcodecs::IMREAD_COLOR)?;

    let mut result = Mat::default();
    opencv::imgproc::match_template_def(
        &img,
        &img_template,
        &mut result,
        opencv::imgproc::TM_SQDIFF,
    )?;

    let mut min_val = 0.0_f64;
    let mut max_val = 0.0_f64;
    let mut min_pos = core::Point::default();
    let mut max_pos = core::Point::default();

    core::min_max_loc(
        &result,
        Some(&mut min_val),
        Some(&mut max_val),
        Some(&mut min_pos),
        Some(&mut max_pos),
        &core::no_array(),
    )?;

    let template_size = img_template.size()?;
    opencv::imgproc::rectangle(
        &mut img,
        opencv::core::Rect::new(
            min_pos.x,
            min_pos.y,
            template_size.width,
            template_size.height,
        ),
        opencv::core::Scalar::new(0., 0., 255., 0.),
        2,
        opencv::imgproc::LINE_4,
        0,
    )?;

    println!("{:?}", min_val);
    println!("{:?}", max_val);
    println!("{:?}", min_pos);
    println!("{:?}", max_pos);

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
