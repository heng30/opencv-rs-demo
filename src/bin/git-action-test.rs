use anyhow::Result;
use opencv::imgcodecs;

fn main() -> Result<()> {
    let _img = imgcodecs::imread("data/test.png", imgcodecs::IMREAD_COLOR)?;
    Ok(())
}
