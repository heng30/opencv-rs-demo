use anyhow::Result;
use opencv::{highgui, imgcodecs};

fn main() -> Result<()> {
    let window_name = "mouse";
    let img = imgcodecs::imread("test.png", imgcodecs::IMREAD_COLOR)?;
    highgui::named_window(window_name, highgui::WINDOW_NORMAL)?;
    highgui::resize_window(window_name, 640, 480)?;
    highgui::imshow(window_name, &img)?;

    highgui::set_mouse_callback(
        window_name,
        Some(Box::new(|event, x, y, flags| {
            println!("event: {event} (x, y): ({x}, {y}) flags: {flags}");
        })),
    )?;

    loop {
        let key = highgui::wait_key(0)?;
        if key & 0xFF == 'q' as i32 {
            break;
        }
    }

    highgui::destroy_all_windows()?;
    Ok(())
}
