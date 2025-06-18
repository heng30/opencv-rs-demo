use anyhow::Result;
use opencv::{core, highgui, imgcodecs, imgproc};

fn main() -> Result<()> {
    let (w, h) = (640, 480);
    let window_name = "img-contours";

    let mut gray = core::Mat::default();
    let img = imgcodecs::imread("data/contours.png", imgcodecs::IMREAD_COLOR)?;

    // Convert to grayscale
    imgproc::cvt_color(
        &img,
        &mut gray,
        imgproc::COLOR_BGR2GRAY,
        0,
        opencv::core::AlgorithmHint::ALGO_HINT_DEFAULT.into(),
    )?;

    // Apply threshold
    let mut binary = core::Mat::default();
    imgproc::threshold(&gray, &mut binary, 160., 255., imgproc::THRESH_BINARY)?;

    // Find contours
    let mut contours = core::Vector::<core::Mat>::new();
    imgproc::find_contours(
        &binary,
        &mut contours,
        imgproc::RETR_TREE,
        imgproc::CHAIN_APPROX_SIMPLE,
        core::Point::default(),
    )?;

    println!("Found {} contours", contours.len());

    for (index, contour) in contours.iter().enumerate() {
        let length = imgproc::arc_length(&contour, true)?;
        println!("contour {index} arch length is {length:.2}");
    }

    // Draw contours on original image
    let mut result = img.clone();
    imgproc::draw_contours(
        &mut result,
        &contours,
        -1, // draw all contours
        core::Scalar::new(0., 0., 255., 0.),
        2, // thickness
        imgproc::LINE_8,
        &core::no_array(),
        2, // maxLevel
        core::Point::default(),
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
