use anyhow::Result;
use opencv::{
    core::{Mat, Vector},
    highgui,
    prelude::*,
    videoio::{VideoCapture, VideoWriter},
};

fn main() -> Result<()> {
    let mut ready_index = Vector::new();
    let mut caps = Vector::new();
    caps.push(VideoCapture::new(0, opencv::videoio::CAP_ANY)?);

    // get all ready index
    if !VideoCapture::wait_any(&caps, &mut ready_index, 0)? {
        anyhow::bail!("can't find camera");
    }

    println!("ready_index: {ready_index:?}");

    let mut entries = vec![];

    // create windows for each camera
    for (index, _) in caps.iter().enumerate() {
        let width = caps
            .get(index)
            .unwrap()
            .get(opencv::videoio::CAP_PROP_FRAME_WIDTH)? as i32;

        let height = caps
            .get(index)
            .unwrap()
            .get(opencv::videoio::CAP_PROP_FRAME_HEIGHT)? as i32;

        // Get the actual frame rate from the camera
        let fps = caps
            .get(index)
            .unwrap()
            .get(opencv::videoio::CAP_PROP_FPS)?;
        let fps = if fps <= 0.0 { 30.0 } else { fps };
        let delay = (1000.0 / fps) as i32;

        println!("camera width and height: ({width}, {height}), FPS: {fps}");

        let fourcc = VideoWriter::fourcc('a', 'v', 'c', '1')?;
        let window_name = format!("camera-{}", index);
        highgui::named_window(&window_name, highgui::WINDOW_NORMAL)?;
        highgui::resize_window(&window_name, width, height)?;

        let vw = VideoWriter::new(
            &format!("target/{window_name}.mp4"),
            fourcc,
            fps,
            opencv::core::Size { width, height },
            true,
        )?;

        if !vw.is_opened()? {
            anyhow::bail!("VideoWriter is not open");
        }

        entries.push((window_name, vw, delay));
    }

    // show cameras' capture
    'out: loop {
        for index in ready_index.iter() {
            let start = std::time::Instant::now();

            let index = index as usize;
            let mut img = Mat::default();

            if let Ok(true) = caps.get(index).unwrap().read(&mut img) {
                highgui::imshow(&entries[index].0, &img)?;
                entries[index].1.write(&img)?;
            }

            let elapsed = start.elapsed().as_millis() as i32;
            let wait_time = std::cmp::max(1, entries[index].2 - elapsed);

            let key = highgui::wait_key(wait_time)?;
            if key & 0xFF == 'q' as i32 {
                break 'out;
            }
        }
    }

    for (mut entry, mut cap) in entries.into_iter().zip(caps.into_iter()) {
        entry.1.release()?;
        cap.release()?;
    }

    highgui::destroy_all_windows()?;

    Ok(())
}
