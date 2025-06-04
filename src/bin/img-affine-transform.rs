use anyhow::Result;
use opencv::{core, highgui, imgcodecs};

fn main() -> Result<()> {
    let (w, h) = (640, 480);
    let window_name = "img-affine-transfrom";

    let img = imgcodecs::imread("data/test.png", imgcodecs::IMREAD_COLOR)?;
    let src_points = core::Mat::from_slice_2d(&[
        [400.0_f32, 300.0_f32],
        [800.0_f32, 300.0_f32],
        [400.0_f32, 1000.0_f32],
    ])?;

    let dst_points = core::Mat::from_slice_2d(&[
        [200.0_f32, 400.0_f32],
        [600.0_f32, 500.0_f32],
        [150.0_f32, 900.0_f32],
    ])?;

    let m = opencv::imgproc::get_affine_transform(&src_points, &dst_points)?;

    let mut result = core::Mat::default();

    opencv::imgproc::warp_affine(
        &img,
        &mut result,
        &m,
        core::Size::new(w, h),
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
