use anyhow::Result;
use opencv::{
    core,
    core::{Mat, ToInputOutputArray},
    highgui,
    prelude::*,
    videoio::{VideoCapture, CAP_ANY},
};

fn main() -> Result<()> {
    let (width, height) = (1280, 720);
    let mut ready_index = core::Vector::new();
    let mut caps = core::Vector::new();
    caps.push(VideoCapture::new(0, CAP_ANY)?);

    // 获取所有可用的摄像头设备编号
    if !VideoCapture::wait_any(&caps, &mut ready_index, 0)? {
        println!("can't find camera");
        return Ok(());
    }

    println!("ready_index: {ready_index:?}");

    // create entries for each camera
    let mut entries = vec![];
    for (index, mut cap) in caps.iter().enumerate() {
        // 设置视频大小
        cap.set(
            opencv::videoio::VideoCaptureProperties::CAP_PROP_FRAME_WIDTH.into(),
            width as f64,
        )?;
        cap.set(
            opencv::videoio::VideoCaptureProperties::CAP_PROP_FRAME_HEIGHT.into(),
            height as f64,
        )?;

        // 获取摄像头帧率
        let fps = cap.get(opencv::videoio::CAP_PROP_FPS)?;
        let fps = if fps <= 0.0 { 30.0 } else { fps };
        let delay = (1000.0 / fps) as i32;

        println!("camera width and height: ({width}, {height}), FPS: {fps}");
        let window_name = format!("camera-{}", index);
        highgui::named_window(&window_name, highgui::WINDOW_NORMAL)?;
        highgui::resize_window(&window_name, width, height)?;
        entries.push((window_name, delay));
    }

    // 播放摄像头
    'out: loop {
        for index in ready_index.iter() {
            let start = std::time::Instant::now();

            let index = index as usize;
            let mut img = Mat::default();

            if let Ok(true) = caps.get(index).unwrap().read(&mut img) {
                core::flip(&img.clone(), &mut img, 1)?;
                let _btns = draw_calculator(&mut img)?;

                // TODO: 使用cvzone 和 mideopipe 进行手掌检测和绘制

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

fn draw_calculator(frame: &mut impl ToInputOutputArray) -> Result<Vec<(&'static str, core::Rect)>> {
    let (x, y, w) = (800, 70, 400);
    let output_box_h = 100;
    let btn_size = w / 4;

    // 绘制输出外框
    opencv::imgproc::rectangle(
        frame,
        core::Rect::new(x, y, w, output_box_h),
        opencv::core::Scalar::new(255., 255., 255., 0.),
        -1,
        opencv::imgproc::LINE_4,
        0,
    )?;

    opencv::imgproc::rectangle(
        frame,
        core::Rect::new(x, y, w, output_box_h),
        opencv::core::Scalar::new(50., 50., 50., 0.),
        2,
        opencv::imgproc::LINE_4,
        0,
    )?;

    // 绘制按钮
    let mut btns = vec![];
    let btns_value = [
        ["7", "8", "9", "*"],
        ["4", "5", "6", "-"],
        ["1", "2", "3", "+"],
        ["0", "/", ".", "="],
    ];

    for (r, row) in btns_value.into_iter().enumerate() {
        for (c, value) in row.into_iter().enumerate() {
            btns.push((
                value,
                core::Rect::new(
                    x + c as i32 * btn_size,
                    y + output_box_h + r as i32 * btn_size,
                    btn_size,
                    btn_size,
                ),
            ));
        }
    }

    for (value, pos_size) in &btns {
        draw_button(frame, value, pos_size.clone())?;
    }

    btns.push(("", core::Rect::new(x, y, w, output_box_h)));
    Ok(btns)
}

fn draw_button(
    frame: &mut impl ToInputOutputArray,
    value: &str,
    pos_size: core::Rect,
) -> Result<()> {
    opencv::imgproc::rectangle(
        frame,
        pos_size.clone(),
        opencv::core::Scalar::new(255., 255., 255., 0.),
        -1,
        opencv::imgproc::LINE_4,
        0,
    )?;

    opencv::imgproc::rectangle(
        frame,
        pos_size.clone(),
        opencv::core::Scalar::new(0., 0., 0., 0.),
        2,
        opencv::imgproc::LINE_4,
        0,
    )?;

    opencv::imgproc::put_text(
        frame,
        value,
        opencv::core::Point::new(pos_size.x + 30, pos_size.y + 70),
        opencv::imgproc::HersheyFonts::FONT_HERSHEY_SIMPLEX.into(),
        2.,
        opencv::core::Scalar::new(0., 0., 0., 0.),
        2,
        opencv::imgproc::LINE_AA,
        false,
    )?;

    Ok(())
}
