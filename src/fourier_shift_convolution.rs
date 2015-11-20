extern crate dft;
#[macro_use(tensor)]
extern crate numeric;
use numeric::{Tensor, AxisIndex};
use dft::{Operation, Plan, Transform, c64};
use std::iter::FromIterator;
use std::f64::consts::PI;

fn main()
{
    let size: usize = 32;
    let planf        = Plan::new(dft::Operation::Forward, size);
    let plani        = Plan::new(dft::Operation::Inverse, size);
    let mut a        = Tensor::new( Vec::from_iter ((0..size) .map(|idx| c64::new((idx+1) as f64, 0.0))));
    let mut b        = tensor![c64::new(0.0, 0.0); size];
    let mut kernel   = Tensor::new( Vec::from_iter ((0..size) .map(|idx|
                                                        c64::new( (2.0*PI * 3.0*(idx as f64) / (size as f64)).cos(),
                                                                  (2.0*PI * 3.0*(idx as f64) / (size as f64)).sin() ))));

    println!("a: {:?}, kernel: {:?}", a.data(), kernel.data());

    //(*a.data()).transform(&planf);
    //let mut data = vec![c64::new(42.0, 69.0); size];
    //data.transform(&planf);
}
