use ndarray::Array2;
use ndarray::Array3;
use opencv::prelude::*;

fn test1() {
    let mat = Mat::from_slice(&[1.0, 2.0, 3.0, 4.0]).unwrap();
    let mat = mat.reshape(1, 2).unwrap();

    println!("{:?}\n", mat);

    let array = Array2::from_shape_vec((2, 2), mat.data_typed::<f64>().unwrap().to_vec()).unwrap();

    let doubled = &array * 2.0;
    println!("{:?}\n", doubled);

    let mat_back = Mat::from_slice(doubled.as_slice().unwrap()).unwrap();
    println!("{mat_back:?}");
}

fn test2() {
    let mat = Mat::from_slice(&[1.0, 2.0, 3.0, 4.0]).unwrap();
    let mat = mat.reshape(2, 2).unwrap(); // 2 channels, 2 rows

    println!("{:?}\n", mat);

    // For 2-channel matrix, we need to use Array3
    let data = mat.data_typed::<opencv::core::Vec2d>().unwrap();
    let array = Array3::from_shape_vec(
        (2, 1, 2),
        data.iter().flat_map(|v| v.as_slice()).copied().collect(),
    )
    .unwrap();

    let doubled = &array * 2.0;
    println!("{:?}\n", doubled);

    // Convert back to Mat
    let mat_back = Mat::from_slice(doubled.as_slice().unwrap()).unwrap();
    let mat_back = mat_back.reshape(2, 2).unwrap();
    println!("{mat_back:?}");
}

fn main() {
    println!("===============test1================");
    test1();

    println!("===============test2================");
    test2();
}
