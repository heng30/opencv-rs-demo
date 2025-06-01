use anyhow::Result;
use opencv::{
    core::{Mat, Scalar},
    highgui,
    prelude::{MatTrait, MatTraitConst},
};
use std::sync::{
    atomic::{AtomicU8, Ordering},
    Arc,
};

fn main() -> Result<()> {
    let (w, h) = (640, 480);
    let window_name = "mouse";
    let (r_value, g_value, b_value) = (
        Arc::new(AtomicU8::new(0_u8)),
        Arc::new(AtomicU8::new(0_u8)),
        Arc::new(AtomicU8::new(0_u8)),
    );

    highgui::named_window(window_name, highgui::WINDOW_NORMAL)?;
    highgui::resize_window(window_name, w, h)?;

    let rv = r_value.clone();
    highgui::create_trackbar(
        "Red",
        window_name,
        None,
        255,
        Some(Box::new(move |v| {
            rv.store(v as u8, Ordering::Relaxed);
        })),
    )?;

    let gv = g_value.clone();
    highgui::create_trackbar(
        "Green",
        window_name,
        None,
        255,
        Some(Box::new(move |v| {
            gv.store(v as u8, Ordering::Relaxed);
        })),
    )?;

    let bv = b_value.clone();
    highgui::create_trackbar(
        "Blue",
        window_name,
        None,
        255,
        Some(Box::new(move |v| {
            bv.store(v as u8, Ordering::Relaxed);
        })),
    )?;

    let mut img = Mat::new_rows_cols_with_default(h, w, opencv::core::CV_8UC3, Scalar::all(0.0))?;

    loop {
        let rv = r_value.load(Ordering::Relaxed);
        let gv = g_value.load(Ordering::Relaxed);
        let bv = b_value.load(Ordering::Relaxed);

        for i in 0..img.rows() {
            for j in 0..img.cols() {
                let b = ((j as f32 / img.cols() as f32) * bv as f32) as u8;
                let g = ((i as f32 / img.rows() as f32) * gv as f32) as u8;
                let r = (((i + j) as f32 / (img.rows() + img.cols()) as f32) * rv as f32) as u8;

                *img.at_2d_mut::<opencv::core::Vec3b>(i, j)? =
                    opencv::core::Vec3b::from_array([b, g, r]);
            }
        }

        highgui::imshow(window_name, &img)?;

        let key = highgui::wait_key(40)?;
        if key & 0xFF == 'q' as i32 {
            break;
        }
    }

    highgui::destroy_all_windows()?;
    Ok(())
}
