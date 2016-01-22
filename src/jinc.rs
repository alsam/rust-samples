#![feature(non_ascii_idents)]

extern crate num;

extern crate special_fun;
extern crate roots;

use special_fun::FloatSpecial;
use std::f64::consts::PI;
use roots::SimpleConvergency;
use roots::find_root_brent;

fn jinc(r: f64) -> f64 {
    let π2r = 2.0f64*PI*r;
    2.0f64*π2r.besselj(1.0) / π2r
}

fn find_roots(a: f64, b: f64, f: &Fn(f64)->f64) -> Vec<f64> {
    Vec::new()
}

fn main() {
    let v = 0.5f64.besselj(1.0);
    println!("besselj(1.0, 0.5) : {}", v);
    println!("jinc(0.61) : {}", jinc(0.609835));

}
