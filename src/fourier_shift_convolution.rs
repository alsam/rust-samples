extern crate dft;
use dft::{Operation, Plan, Transform, c64};
use std::iter::FromIterator;
use std::f64::consts::PI;

fn main()
{
    let size: usize  = 32;
    let planf        = Plan::new(dft::Operation::Forward, size);
    let plani        = Plan::new(dft::Operation::Inverse, size);
    let mut a        = Vec::from_iter ((0..size) .map(|idx| c64::new((idx+1) as f64, 0.0)));
    let mut kernel   = Vec::from_iter ((0..size) .map(|idx| c64::new( (2.0*PI * 3.0*(idx as f64) / (size as f64)).cos(),
                                                                      (2.0*PI * 3.0*(idx as f64) / (size as f64)).sin() )));
    println!("a: {:?}, kernel: {:?}", a, kernel);

    // forward Fourier transform
    a.transform(&planf);

    // convolution with the kernel
    let mut b        = Vec::from_iter ((0..size) .map(|i| a[i] * kernel[i]));

    // inverse Fourier transform
    b.transform(&plani);
    println!("b: {:?}", b);

    // expected cyclic shift:
    // a = [1, 2, 3, ...]
    // b = [4, 5, 6, ...]
}
