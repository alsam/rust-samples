#![allow(unused)]
#[macro_use(s)]
extern crate ndarray;
extern crate num_traits;

use num_traits::Float;

use ndarray::{
    ArrayView,
    ArrayViewMut,
    Array,
    ArrayD,
    Ix,
    IxDyn,
    arr2,
};

type Ix2 = (Ix, Ix);

fn main() {
    let n = 16;
    let mut a = Array::zeros((n, n));
 
    for ((i, j), elt) in a.indexed_iter_mut() {
 
        *elt = (i as f32).powi(2) + (j as f32).powi(2);
    
    }
    println!("{:3}", a);

    // an array with the dynamic dimension type
    let mut b: ArrayD<f64>;
    // 5 × 6 × 3 × 4 array using the dynamic dimension type
    b = ArrayD::<f64>::zeros(IxDyn(&[5, 6, 3, 4]));
    b[[1, 2, 1, 2]] = 42.;
    println!("b: {:?}", b);
    // reshape to 30 × 3 × 4 array
    let c = b.into_shape((30, 3, 4)).clone();
    println!("reshaped b: {:?}", c);

    let mut ma = arr2(&[[1.,2.,3.], [4.,5.,6.]]);
    println!("ma: {:?}", ma);
    println!("ma.t(): {:?}", ma.t());
    ma = ma.reversed_axes();
    println!("ma: {:?}", ma);
}
