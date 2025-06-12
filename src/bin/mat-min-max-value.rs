use anyhow::Result;
use opencv::core;

fn main() -> Result<()> {
    let mat = core::Mat::from_slice_2d(&[&[1.0f64, 2.0, 3.0], &[4.0, 5.0, 6.0], &[7.0, 9.0, 8.0]])?;

    let mut min_val = 0.0_f64;
    let mut max_val = 0.0_f64;
    core::min_max_loc(
        &mat,
        Some(&mut min_val),
        Some(&mut max_val),
        None,
        None,
        &core::no_array(),
    )?;
    println!("Min value: {:?}", min_val);
    println!("Max value: {:?}", max_val);
    Ok(())
}
