use anyhow::Result;
use opencv::{
    core::{Mat, Vector},
    highgui,
    prelude::*,
    videoio::{VideoCapture, CAP_V4L2},
};

fn main() -> Result<()> {
    let mut ready_index = Vector::new();
    let mut caps = Vector::new();
    caps.push(VideoCapture::new(0, CAP_V4L2)?);

    // get all ready index
    if !VideoCapture::wait_any(&caps, &mut ready_index, 0)? {
        println!("can't find camera");
        return Ok(());
    }

    println!("ready_index: {ready_index:?}");

    // create windows for each camera
    let mut windows = vec![];
    for (index, _) in caps.iter().enumerate() {
        let window_name = format!("capvideo-{}", index);
        highgui::named_window(&window_name, highgui::WINDOW_NORMAL)?;
        highgui::resize_window(&window_name, 640, 480)?;
        windows.push(window_name);
    }

    // show cameras' capture
    'out: loop {
        for index in ready_index.iter() {
            let index = index as usize;
            let mut img = Mat::default();

            if let Ok(true) = caps.get(index).unwrap().read(&mut img) {
                highgui::imshow(&windows[index], &img)?;
            }

            let key = highgui::wait_key(5)?;
            if key & 0xFF == 'q' as i32 {
                break 'out;
            }
        }
    }

    for (index, _) in caps.iter().enumerate() {
        caps.get(index as usize).unwrap().release()?;
    }

    highgui::destroy_all_windows()?;

    Ok(())
}
