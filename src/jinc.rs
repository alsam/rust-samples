// #![feature(plugin)]
// #![feature(non_ascii_idents)]
// #![plugin(indoc)]

// see [Conditional compilation of feature gates?](https://users.rust-lang.org/t/conditional-compilation-of-feature-gates/4765)
#![cfg_attr(feature = "use-nightly", feature(non_ascii_idents))]

extern crate unindent;
use unindent::unindent;

extern crate num;

extern crate special_fun;
extern crate roots;
extern crate integration;

use std::io;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::f64::consts::PI;
use std::iter::FromIterator;
use special_fun::FloatSpecial;
use roots::SimpleConvergency;
use roots::find_root_brent;
use integration::quadrature;

/**
 * Returns a @value \f$\frac{J_1(2\pi r)}{2\pi r}\f$ where \f$J_1\f$ - Bessel function of first kind. ... */
#[cfg(feature = "use-nightly")]
fn jinc(r: f64) -> f64 {
    let π2r = 2.0f64*PI*r;
    2.0f64*π2r.besselj(1.0) / π2r
}

#[cfg(not(feature = "use-nightly"))]
fn jinc(r: f64) -> f64 {
    let pi2r = 2.0f64*PI*r;
    2.0f64*pi2r.besselj(1.0) / pi2r
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

#[inline]
fn definite_integral(f: &Fn(f64) -> f64, a: f64, b: f64) -> f64 {
    quadrature::gauss_legendre(&f, a, b, 1e-12)
}
 
fn write_plot_asy(fname: &str, x_label: &str, y_label: &str, x: &[f64], y: &[f64]) -> Result<(), io::Error> {
    let mut f = try!(File::create(fname));
    //try!(f.write(indoc!("
    try!(f.write(unindent("
                 import graph;
                 
                 size(600,600,IgnoreAspect);
                 
                 real[] x = {").as_bytes()));

    for x_elem in x {
        try!(write!(f, "{}, ", x_elem));
    }

    //try!(f.write(indoc!("
    try!(f.write(unindent("
                 };
                 real[] y = {").as_bytes()));

    for y_elem in y {
        try!(write!(f, "{}, ", y_elem));
    }

    //try!(f.write(indoc!(r#"
    try!(f.write(unindent(r#"
                 };
                 
                 draw(graph(x, y),  green, "", MarkFill[0]);
                 xaxis("r", BottomTop());
                 yaxis(rotate(90)*"$\int_{z=0}^{z=\textbf{r}}J_1(z)dz$", LeftRight(), RightTicks(Label(fontsize(6pt)),
                       new real[]{0, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0, 1.1, 1.2, 1.3, 1.4, 1.5, 1.6, 1.7}) );
                 
                 yequals(1, red+Dotted);
                 
                 "#).as_bytes()));

    Ok(())

}

fn main() {
    let v = 0.5f64.besselj(1.0);
    println!("besselj(1.0, 0.5) : {}", v);
    println!("jinc(0.61) : {}", jinc(0.609835));

    let jinc_roots = find_roots(0f64, 5f64, &jinc);
    println!("jinc roots : {:?}", jinc_roots);
    println!("f({}) = {}", 1000., f(1000.));
    println!("f({}) = {}", 3000., f(3000.));

    //const NPOINTS : usize = 350;
    const NPOINTS : usize = 2750;
    const SUBBANDS : usize = 20;
    let x = Vec::from_iter ( (0..NPOINTS*SUBBANDS) .map(|i| (i as f64) / (SUBBANDS as f64)) );
    //let y = Vec::from_iter ( (0..NPOINTS*SUBBANDS) .map(|i| f(x[i]) ) );
    let partial_sums = Vec::from_iter ( (0..NPOINTS*SUBBANDS - 1) .map(|i|  definite_integral(&|z: f64| z.besselj(1.0), x[i], x[i+1]  ) ) );
    let mut y = Vec::from_iter ( partial_sums.iter().scan(0.0f64, |state, &x| { *state = *state + x; Some(*state) } ) );
    y.insert(0, 0f64); // TODO insert is not very efficient
    assert!(x.len() == y.len(), "x.len() != y.len()" );

    write_plot_asy("J1_int.asy", "", "", &x, &y);

}
