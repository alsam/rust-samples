extern crate num;
use std::f32;
use num::{Num, Zero, Float, Complex};

type c32 = num::Complex<f32>;
type c64 = num::Complex<f64>;

trait ApproxZero {
    fn is_approx_zero<T: Float>(&self, eps: T) -> bool;
}

impl ApproxZero for f32 {
    fn is_approx_zero<T: Float>(&self, eps: T) -> bool {
        self.abs() < eps.to_f32().unwrap().abs()
    }
}

impl ApproxZero for c32 {
    fn is_approx_zero<T: Float>(&self, eps: T) -> bool {
        self.norm_sqr() < eps.to_f32().unwrap().powi(2)
    }
}


fn main() {
    let f32_num = 1.0e-12;
    let c32_num = Complex::new(f32_num, f32_num);
    let eps = 1e-7;
    println!("f32_num : {:} c32_num : {:} eps : {:} {:} {:}",
              f32_num, c32_num, eps,
              f32_num.is_approx_zero(eps),
              c32_num.is_approx_zero(eps));
}
