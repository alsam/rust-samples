extern crate dft;
use dft::{Operation, Plan, Transform, c64};
use std::iter::FromIterator;
use std::f64::consts::PI;

extern crate arrayfire as af;

use af::*;

fn main() {
    let size: usize = 32;
    let planf = Plan::new(dft::Operation::Forward, size);
    let plani = Plan::new(dft::Operation::Inverse, size);
    let mut a = Vec::from_iter((0..size).map(|idx| c64::new((idx + 1) as f64, 0.0)));
    let mut kernel = Vec::from_iter((0..size).map(|idx| {
        c64::new((2.0 * PI * 3.0 * (idx as f64) / (size as f64)).cos(),
                 (2.0 * PI * 3.0 * (idx as f64) / (size as f64)).sin())
    }));
    println!("a: {:?}, kernel: {:?}", a, kernel);

    // forward Fourier transform
    a.transform(&planf);

    // convolution with the kernel
    let mut b = Vec::from_iter((0..size).map(|i| a[i] * kernel[i]));

    println!("b before ifft : {:?}", b);

    // inverse Fourier transform
    b.transform(&plani);
    println!("b: {:?}", b);

    // expected cyclic shift:
    // a = [1, 2, 3, ...]
    // b = [4, 5, 6, ...]

    // play with ArrayFire
    // only CPU
    set_backend(Backend::AF_BACKEND_CPU);
    let mut int_values = Vec::from_iter((0..size).map(|idx| idx + 1));
    let cta_grid = Dim4::new(&[size as u64, 1, 1, 1]);
    let af = Array::new(cta_grid,
        &Vec::from_iter(int_values.iter().map (|&i| c64::new(i as f64, 0.0) ) ), Aftype::C64).unwrap();

    println!("the constructed array has {} elements", af.elements().unwrap());
    print(&af);

    let kernelf = Array::new(cta_grid, &kernel, Aftype::C64).unwrap();
    println!("kernelf:");
    print(&kernelf);

    let mut bf = fft_convolve1(&af, &kernelf, ConvMode::DEFAULT).unwrap();

    println!("ArrayFire convolved array:");
    print(&bf);

    let af_transformed = fft(&af, 1.0, size as i64).unwrap();
    let convolved1 = convolve1(&af_transformed, &kernelf, ConvMode::DEFAULT, ConvDomain::AUTO).unwrap();
    println!("convolved1:");
    print(&convolved1);

}
