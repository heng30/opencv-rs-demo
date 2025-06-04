use anyhow::Result;
use opencv::{core, highgui, imgcodecs};

fn main() -> Result<()> {
    let (w, h) = (640, 480);
    let window_name = "img-affine";

    let img = imgcodecs::imread("data/test.png", imgcodecs::IMREAD_COLOR)?;
    let m = core::Mat::from_slice_2d(&[&[1., 0., 100.], &[0., 1., 200.]])?;
    let mut result = core::Mat::default();

    opencv::imgproc::warp_affine(
        &img,
        &mut result,
        &m,
        core::Size::new(w, h),
        opencv::imgproc::INTER_LINEAR,
        core::BORDER_CONSTANT,
        core::Scalar::all(255.),
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
