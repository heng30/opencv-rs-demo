use anyhow::Result;
use opencv::{
    core::{Mat, Vector},
    highgui,
    prelude::*,
    videoio::{VideoCapture, CAP_ANY},
};

fn main() -> Result<()> {
    let mut ready_index = Vector::new();
    let mut caps = Vector::new();
    caps.push(VideoCapture::new(0, CAP_ANY)?);

    // get all ready index
    if !VideoCapture::wait_any(&caps, &mut ready_index, 0)? {
        println!("can't find camera");
        return Ok(());
    }

    println!("ready_index: {ready_index:?}");

    // create entries for each camera
    let mut entries = vec![];
    for (index, cap) in caps.iter().enumerate() {
        let width = cap.get(opencv::videoio::CAP_PROP_FRAME_WIDTH)? as i32;

        let height = cap.get(opencv::videoio::CAP_PROP_FRAME_HEIGHT)? as i32;

        // Get the actual frame rate from the camera
        let fps = cap.get(opencv::videoio::CAP_PROP_FPS)?;
        let fps = if fps <= 0.0 { 30.0 } else { fps };
        let delay = (1000.0 / fps) as i32;

        println!("camera width and height: ({width}, {height}), FPS: {fps}");
        let window_name = format!("camera-{}", index);
        highgui::named_window(&window_name, highgui::WINDOW_NORMAL)?;
        highgui::resize_window(&window_name, width, height)?;
        entries.push((window_name, delay));
    }

    // show cameras' capture
    'out: loop {
        for index in ready_index.iter() {
            let start = std::time::Instant::now();

            let index = index as usize;
            let mut img = Mat::default();

            if let Ok(true) = caps.get(index).unwrap().read(&mut img) {
                highgui::imshow(&entries[index].0, &img)?;
            }

            let elapsed = start.elapsed().as_millis() as i32;
            let wait_time = std::cmp::max(1, entries[index].1 - elapsed);

            let key = highgui::wait_key(wait_time)?;
            if key & 0xFF == 'q' as i32 {
                break 'out;
            }
        }
    }

    for mut cap in caps.into_iter() {
        cap.release()?;
    }

    highgui::destroy_all_windows()?;

    Ok(())
}
