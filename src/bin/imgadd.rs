use anyhow::Result;
use opencv::{core, highgui, imgcodecs, prelude::*};

fn main() -> Result<()> {
    let (w, h) = (640, 480);
    let window_name = "imgadd";
    let img = imgcodecs::imread("data/color.png", imgcodecs::IMREAD_COLOR)?;

    let rows = img.rows();
    let cols = img.cols();
    let channels = img.channels();
    println!("{rows}, {cols}, {channels}");

    if channels != 3 {
        anyhow::bail!("data/color.png channels is not 3");
    }

    let mask = Mat::ones(rows, cols, core::CV_8UC3)?
        .to_mat()?
        .mul(&200., 1.)?;

    let img_add = (img + mask).into_result()?;

    highgui::named_window(window_name, highgui::WINDOW_NORMAL)?;
    highgui::resize_window(window_name, w, h)?;

    loop {
        highgui::imshow(window_name, &img_add)?;

        let key = highgui::wait_key(0)?;
        if key & 0xFF == 'q' as i32 {
            break;
        }
    }

    highgui::destroy_all_windows()?;
    Ok(())
}
