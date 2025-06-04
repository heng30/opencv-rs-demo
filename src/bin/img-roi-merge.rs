use anyhow::Result;
use opencv::{core, highgui, imgcodecs, prelude::*};

fn main() -> Result<()> {
    let (w, h) = (640, 480);
    let window_name = "img-cut-merge";
    let img1 = imgcodecs::imread("data/color.png", imgcodecs::IMREAD_COLOR)?;
    let img2 = imgcodecs::imread("data/test.png", imgcodecs::IMREAD_COLOR)?;

    if img1.size()? != img2.size()? {
        anyhow::bail!("Images must have the same dimensions");
    }

    // Define ROI (Region of Interest) where to place the overlay
    let roi = core::Rect::new(200, 200, 500, 500);
    let roi_img1 = core::Mat::roi(&img1, roi)?;
    let roi_img2 = core::Mat::roi(&img2, roi)?;

    let mut result = core::Mat::default();
    core::add_weighted(&roi_img1, 0.8, &roi_img2, 0.2, 0.0, &mut result, -1)?;

    highgui::named_window(window_name, highgui::WINDOW_NORMAL)?;
    highgui::resize_window(window_name, w, h)?;

    loop {
        highgui::imshow(window_name, &result)?;

        let key = highgui::wait_key(0)?;
        if key & 0xFF == 'q' as i32 {
            break;
        }
    }

    highgui::destroy_all_windows()?;
    Ok(())
}
