#![feature(non_ascii_idents)]

extern crate num;

extern crate special_fun;
extern crate roots;
extern crate integration;

use special_fun::FloatSpecial;
use std::f64::consts::PI;
use roots::SimpleConvergency;
use roots::find_root_brent;
use integration::quadrature;

fn jinc(r: f64) -> f64 {
    let π2r = 2.0f64*PI*r;
    2.0f64*π2r.besselj(1.0) / π2r
}

fn find_roots(a: f64, b: f64, f: &Fn(f64)->f64) -> Vec<f64> {
    let mut roots = Vec::new();
    const RANGES : i32 = 100;
    let dx = (b - a) / RANGES as f64;
    let conv = SimpleConvergency{eps:1e-12f64, max_iter:80};
    for r in 0..RANGES {
        let beg = a + (r as f64)*dx;
        let maybe_root = find_root_brent(beg, beg+dx, &f, &conv).ok();
        //match maybe_root {
        //    Some(root) => roots.push(root),
        //    None => {}
        //}
        if maybe_root.is_some() {
            roots.push(maybe_root.unwrap());
        }
    }

    roots
}

fn f(x: f64) -> f64 {
    quadrature::gauss_legendre(&|z: f64| z.besselj(1.0), 0.0, x, 1e-12)
}

fn main() {
    let v = 0.5f64.besselj(1.0);
    println!("besselj(1.0, 0.5) : {}", v);
    println!("jinc(0.61) : {}", jinc(0.609835));

    let jinc_roots = find_roots(0f64, 5f64, &jinc);
    println!("jinc roots : {:?}", jinc_roots);
    println!("f({}) = {}", 1000., f(1000.));
    println!("f({}) = {}", 3000., f(3000.));
}
