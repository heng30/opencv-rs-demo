use anyhow::Result;
use opencv::highgui;
use opencv::prelude::*;

fn main() -> Result<()> {
    let (w, h) = (640, 480);
    let mut img = Mat::zeros(h, w, opencv::core::CV_8UC3)?.to_mat()?;

    let points = opencv::core::Vector::from_slice(&[
        opencv::core::Point::new(50, 50),
        opencv::core::Point::new(500, 300),
        opencv::core::Point::new(100, 400),
    ]);

    let mut pts = opencv::core::Vector::<opencv::core::Vector<opencv::core::Point>>::new();
    pts.push(points);

    opencv::imgproc::polylines(
        &mut img,
        &pts,
        true,
        opencv::core::Scalar::new(0., 255., 255., 0.),
        1,
        opencv::imgproc::LINE_AA,
        0,
    )?;

    opencv::imgproc::fill_poly(
        &mut img,
        &pts,
        opencv::core::Scalar::new(0., 255., 255., 0.),
        opencv::imgproc::LINE_AA,
        0,
        opencv::core::Point::new(0, 0),
    )?;

    let window_name = "draw-polylines";
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
