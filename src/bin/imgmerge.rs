use anyhow::Result;
use opencv::{core, highgui, imgcodecs, prelude::*};

fn main() -> Result<()> {
    let (w, h) = (640, 480);
    let window_name = "imgmerge";
    let img1 = imgcodecs::imread("color.png", imgcodecs::IMREAD_COLOR)?;
    let img2 = imgcodecs::imread("test.png", imgcodecs::IMREAD_COLOR)?;

    if img1.size()? != img2.size()? {
        anyhow::bail!("Images must have the same dimensions");
    }

    // Alpha blending with weights (0.7 for img1, 0.3 for img2)
    let mut result = core::Mat::default();
    core::add_weighted(&img1, 0.7, &img2, 0.3, 0.0, &mut result, -1)?;

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
