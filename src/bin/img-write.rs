use anyhow::Result;
use opencv::core::Vector;
use opencv::{highgui, imgcodecs};

fn main() -> Result<()> {
    let input_img = "test.png";
    let output_img = "target/output.jpeg";

    let img = imgcodecs::imread(input_img, imgcodecs::IMREAD_COLOR)?;
    highgui::imshow("window", &img)?;

    loop {
        let key = highgui::wait_key(10000)?;

        if key & 0xFF == 'q' as i32 {
            break;
        } else if key & 0xFF == 'w' as i32 {
            imgcodecs::imwrite(
                output_img,
                &img,
                &Vector::from_slice(&[0_i32, imgcodecs::IMWRITE_JPEG_QUALITY]),
            )?;
            println!("write {input_img} to {output_img}");
        }
    }

    highgui::destroy_all_windows()?;

    Ok(())
}
