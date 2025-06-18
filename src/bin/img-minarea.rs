use anyhow::Result;
use opencv::{core, highgui, imgcodecs, imgproc, prelude::*};

fn main() -> Result<()> {
    let (w, h) = (640, 480);
    let window_name = "img-min-area";

    let mut gray = core::Mat::default();
    let mut img = imgcodecs::imread("data/hello.png", imgcodecs::IMREAD_COLOR)?;

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
    imgproc::threshold(&gray, &mut binary, 120., 255., imgproc::THRESH_BINARY)?;

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
    imgproc::draw_contours(
        &mut img,
        &contours,
        1, // draw all contours
        core::Scalar::new(0., 0., 255., 0.),
        2, // thickness
        imgproc::LINE_8,
        &core::no_array(),
        2, // maxLevel
        core::Point::default(),
    )?;

    // Draw min rectangel for contours[1]
    let min_rect = opencv::imgproc::min_area_rect(&contours.get(1).unwrap())?;

    let mut min_rect_pts = core::Mat::default();
    opencv::imgproc::box_points(min_rect, &mut min_rect_pts)?;

    // println!("{:?}", min_rect_pts);

    // Convert the Mat to Vector<Point> if needed for polylines
    let mut points_vec = core::Vector::<core::Point>::new();
    for i in 0..min_rect_pts.rows() {
        let x = min_rect_pts.at_2d::<f32>(i, 0)?;
        let y = min_rect_pts.at_2d::<f32>(i, 1)?;
        points_vec.push(core::Point::new(x.round() as i32, y.round() as i32));
    }

    let mut pts = core::Vector::<core::Vector<core::Point>>::new();
    pts.push(points_vec);

    opencv::imgproc::polylines(
        &mut img,
        &pts,
        true,
        opencv::core::Scalar::new(255., 0., 0., 0.),
        8,
        opencv::imgproc::LINE_AA,
        0,
    )?;

    highgui::named_window(window_name, highgui::WINDOW_NORMAL)?;
    highgui::resize_window(window_name, w, h)?;

    loop {
        highgui::imshow(window_name, &img)?;

        let key = highgui::wait_key(0)?;
        if key & 0xFF == 'q' as i32 {
            break;
        }
    }

    highgui::destroy_all_windows()?;
    Ok(())
}
