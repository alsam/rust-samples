extern crate num;

extern crate special_fun;
extern crate roots;

use special_fun::FloatSpecial;
use std::f64::consts::PI;
use roots::find_root_secant;

fn jinc(r: f64) -> f64 {
    let pi2r = 2.0f64*PI*r;
    //2.0f64*1.0f64.besselj(pi2r) / pi2r
    2.0f64*pi2r.besselj(1.0f64) / pi2r
}

fn main() {
    let v = 0.5f64.besselj(1.0);
    println!("besselj(1.0, 0.5) : {}", v);
    println!("jinc(0.61) : {}", jinc(0.609835));

}
