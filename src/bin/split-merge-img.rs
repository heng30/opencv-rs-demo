use anyhow::Result;
use opencv::prelude::*;
use opencv::{core, highgui};

fn main() -> Result<()> {
    let (w, h) = (640, 480);
    let img = Mat::zeros(h, w, core::CV_8UC3)?;
    let mut dst_img = Mat::default();

    // Create vector to hold the channels
    let mut channels = core::Vector::<Mat>::new();
    core::split(&img, &mut channels)?;

    if channels.len() != 3 {
        anyhow::bail!("Expected 3 channels but got {}", channels.len());
    }

    // Need to get mutable references to the channels to modify them
    let mut b_channel = channels.get(0)?;
    let mut g_channel = channels.get(1)?;
    let mut r_channel = channels.get(2)?;

    let b_rect = core::Rect::new(50, 50, 200, 200);
    let g_rect = core::Rect::new(80, 80, 200, 200);
    let r_rect = core::Rect::new(30, 30, 200, 200);

    for y in b_rect.y..b_rect.y + b_rect.height {
        for x in b_rect.x..b_rect.x + b_rect.width {
            *b_channel.at_2d_mut::<u8>(y, x)? = 255;
        }
    }

    for y in g_rect.y..g_rect.y + g_rect.height {
        for x in g_rect.x..g_rect.x + g_rect.width {
            *g_channel.at_2d_mut::<u8>(y, x)? = 255;
        }
    }

    for y in r_rect.y..r_rect.y + r_rect.height {
        for x in r_rect.x..r_rect.x + r_rect.width {
            *r_channel.at_2d_mut::<u8>(y, x)? = 255;
        }
    }

    // Merge the channels back
    core::merge(&channels, &mut dst_img)?;

    let window_name = "split-merge";
    highgui::named_window(window_name, highgui::WINDOW_NORMAL)?;
    highgui::resize_window(window_name, w, h)?;

    highgui::imshow(window_name, &dst_img)?;

    loop {
        let key = highgui::wait_key(0)?;
        if key & 0xFF == 'q' as i32 {
            break;
        }
    }

    highgui::destroy_all_windows()?;

    Ok(())
}
