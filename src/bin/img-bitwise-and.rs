use anyhow::Result;
use opencv::{core, highgui, imgcodecs};

fn main() -> Result<()> {
    let (w, h) = (640, 480);
    let window_name = "img-bitwise-and";
    let img1 = imgcodecs::imread("data/test.png", imgcodecs::IMREAD_COLOR)?;
    let img2 = imgcodecs::imread("data/color.png", imgcodecs::IMREAD_COLOR)?;

    let mut result = core::Mat::default();
    core::bitwise_and(&img1, &img2, &mut result, &core::no_array())?;

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
