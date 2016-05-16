#![allow(unused)]
#[macro_use(s)]
extern crate ndarray;
extern crate num_traits;

use num_traits::Float;

use ndarray::{
    ArrayView,
    ArrayViewMut,
    OwnedArray,
    Ix,
};

type Ix2 = (Ix, Ix);

fn main() {
    let n = 16;
    let mut a = OwnedArray::zeros((n, n));
 
    for ((i, j), elt) in a.indexed_iter_mut() {
 
        *elt = ((i as f32).powi(2) + (j as f32).powi(2));
    
    }
    println!("{:3}", a);

}
