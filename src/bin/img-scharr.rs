use anyhow::Result;
use opencv::{core, highgui, imgcodecs};

fn main() -> Result<()> {
    let (w, h) = (640, 480);
    let window_name = "img-scharr";

    let img = imgcodecs::imread("data/lena.png", imgcodecs::IMREAD_COLOR)?;

    let mut result_x = core::Mat::default();
    opencv::imgproc::scharr(&img, &mut result_x, -1, 1, 0, 1., 0., core::BORDER_DEFAULT)?;

    let mut result_y = core::Mat::default();
    opencv::imgproc::scharr(&img, &mut result_y, -1, 0, 1, 1., 0., core::BORDER_DEFAULT)?;

    let result = (result_x + result_y).into_result()?;

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
