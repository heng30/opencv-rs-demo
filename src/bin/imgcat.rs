use anyhow::Result;
use opencv::{core, highgui, imgcodecs, prelude::*};

fn main() -> Result<()> {
    let (w, h) = (640, 480);
    let window_name = "imgmerge";
    let img1 = imgcodecs::imread("data/color.png", imgcodecs::IMREAD_COLOR)?;
    let img2 = imgcodecs::imread("data/test.png", imgcodecs::IMREAD_COLOR)?;

    if img1.size()? != img2.size()? {
        anyhow::bail!("Images must have the same dimensions");
    }

    let mut h_result1 = core::Mat::default();
    core::hconcat2(&img1, &img2, &mut h_result1)?;

    let mut h_result2 = core::Mat::default();
    core::hconcat2(&img2, &img1, &mut h_result2)?;

    let mut result = core::Mat::default();
    core::vconcat2(&h_result1, &h_result2, &mut result)?;

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
