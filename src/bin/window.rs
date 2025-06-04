use anyhow::Result;
use opencv::{highgui, imgcodecs};

fn main() -> Result<()> {
    let window_name = "window";
    let img = imgcodecs::imread("data/test.png", imgcodecs::IMREAD_COLOR)?;
    highgui::named_window(window_name, highgui::WINDOW_NORMAL)?;
    highgui::resize_window(window_name, 640, 480)?;
    highgui::imshow(window_name, &img)?;

    loop {
        let key = highgui::wait_key(10000)?;
        if key & 0xFF == 'q' as i32 {
            break;
        }
    }

    highgui::destroy_all_windows()?;
    Ok(())
}
