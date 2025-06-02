use anyhow::Result;
use opencv::highgui;
use opencv::prelude::*;

fn main() -> Result<()> {
    let (w, h) = (640, 480);
    let mut img = Mat::zeros(h, w, opencv::core::CV_8UC3)?.to_mat()?;

    opencv::imgproc::put_text(
        &mut img,
        "Hello World!",
        opencv::core::Point::new(100, 200),
        opencv::imgproc::HersheyFonts::FONT_HERSHEY_COMPLEX.into(),
        2.,
        opencv::core::Scalar::new(0., 255., 255., 0.),
        1,
        opencv::imgproc::LINE_AA,
        false,
    )?;

    let mut base_line = 0;
    let text_size = opencv::imgproc::get_text_size(
        "Hello World!",
        opencv::imgproc::HersheyFonts::FONT_HERSHEY_PLAIN.into(),
        2.,
        1,
        &mut base_line,
    )?;
    println!("text size: {text_size:?}");

    let window_name = "draw-text";
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
