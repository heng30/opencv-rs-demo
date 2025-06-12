use anyhow::Result;
use opencv::{core, highgui, imgcodecs, prelude::*};

fn main() -> Result<()> {
    let (w, h) = (640, 480);
    let window_name = "img-prydown";
    let img = imgcodecs::imread("data/test.png", imgcodecs::IMREAD_COLOR)?;

    let mut output = core::Mat::default();
    opencv::imgproc::pyr_down_def(&img, &mut output)?;

    highgui::named_window(window_name, highgui::WINDOW_NORMAL)?;
    highgui::resize_window(window_name, w, h)?;
    highgui::imshow(window_name, &output)?;

    println!("original img shape: {:?}", img.size());
    println!("output img shape: {:?}", output.size());

    loop {
        let key = highgui::wait_key(0)?;
        if key & 0xFF == 'q' as i32 {
            break;
        }
    }

    highgui::destroy_all_windows()?;
    Ok(())
}
