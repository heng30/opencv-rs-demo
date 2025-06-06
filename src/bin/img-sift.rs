use anyhow::Result;
use opencv::{core, core::Mat, highgui, imgcodecs, imgproc, prelude::*};

fn main() -> Result<()> {
    let (w, h) = (640, 480);
    let window_name = "img-sift";

    let mut img = imgcodecs::imread("data/chess.png", imgcodecs::IMREAD_COLOR)?;

    // 灰度
    let mut gray = Mat::default();
    imgproc::cvt_color(&img, &mut gray, imgproc::COLOR_BGR2GRAY, 0)?;

    let mut sift = opencv::features2d::SIFT::create_def()?;
    let mut kps = core::Vector::<core::KeyPoint>::new();
    sift.detect_def(&gray, &mut kps)?;

    opencv::features2d::draw_keypoints_def(&img.clone(), &kps, &mut img)?;

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
