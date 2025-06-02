use ndarray::Array2;
use opencv::prelude::*;

fn main() {
    let mat = Mat::from_slice(&[1.0, 2.0, 3.0, 4.0]).unwrap();
    let mat = mat.reshape(1, 4).unwrap();

    println!("{:?}\n", mat);

    let array = Array2::from_shape_vec((2, 2), mat.data_typed::<f64>().unwrap().to_vec()).unwrap();

    let doubled = &array * 2.0;
    println!("{:?}\n", doubled);

    let mat_back = Mat::from_slice(doubled.as_slice().unwrap()).unwrap();
    println!("{mat_back:?}");
}
