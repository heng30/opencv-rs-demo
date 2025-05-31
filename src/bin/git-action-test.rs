use anyhow::Result;
use opencv::imgcodecs;

fn main() -> Result<()> {
    let _img = imgcodecs::imread("test.png", imgcodecs::IMREAD_COLOR)?;
    Ok(())
}
