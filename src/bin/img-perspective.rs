use anyhow::Result;
use opencv::{core, highgui, imgcodecs};

fn main() -> Result<()> {
    let (w, h) = (640, 480);
    let window_name = "img-perspective";

    let img = imgcodecs::imread("data/test2.png", imgcodecs::IMREAD_COLOR)?;
    let src_points = core::Mat::from_slice_2d(&[
        [60.0_f32, 440.0_f32],
        [900.0_f32, 440.0_f32],
        [0.0_f32, 1600.0_f32],
        [1200.0_f32, 1550.0_f32],
    ])?;

    let dst_points = core::Mat::from_slice_2d(&[
        [0.0_f32, 0.0_f32],
        [1050.0_f32, 0.0_f32],
        [0.0_f32, 1600.0_f32],
        [1200.0_f32, 1600.0_f32],
    ])?;

    let m = opencv::imgproc::get_perspective_transform(&src_points, &dst_points, core::DECOMP_LU)?;

    let mut result = core::Mat::default();

    opencv::imgproc::warp_perspective(
        &img,
        &mut result,
        &m,
        core::Size::new(1100, 1600),
        opencv::imgproc::INTER_LINEAR,
        core::BORDER_CONSTANT,
        core::Scalar::all(0.),
    )?;

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
