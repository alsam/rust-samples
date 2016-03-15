//***********************************************************************
//  Copyright 2006 John A. Trangenstein
//
//  This software is made available for research and instructional use
//  only.e available for research and instructional use
//  only.
//  You may copy and use this software without charge for these
//  non-commercial purposes, provided that the copyright notice and
//  associated text is reproduced on all copies.
//  For all other uses (including distribution of modified versions),
//  please contact the author at
//    John A. Trangenstein
//    Department of Mathematics
//    Duke University
//    Durham, NC 27708-0320
//    USA
//  or
//    johnt@math.duke.edu
//
//  This software is made available "as is" without any assurance that it
//  is completely correct, or that it will work for your purposes.
//  Use the software at your own risk.
//***********************************************************************

// Permission to publish the translation of the code
// was graciously granted by Prof. John A. Trangenstein

// The MIT License (MIT)
//
// Copyright (c) 2015 Alexander Samoilov
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

#![feature(duration_span)]

extern crate argparse;
extern crate num;
#[macro_use(tensor)]
extern crate numeric;

// http://numeric.rs/doc/numeric/tensor/struct.Tensor.html
use numeric::{Tensor, AxisIndex};

use num::{Num, Zero, One, Signed};

use std::time::Duration;
use std::cmp;
use std::process::exit;
use argparse::{ArgumentParser, StoreTrue, Store};

struct Options {
    verbose: bool,
    num_steps: usize,
    num_cells: usize,
    num_runs: usize,
}

fn min_dx(x: &[f64]) -> f64 {
    // https://doc.rust-lang.org/std/primitive.slice.html#method.windows
    let tdx = Tensor::new( x.windows(2) // iterator for adjacent pairs of a slice
                            .map(|w| w[1]-w[0]) // i.e. [x[0], x[1]], [x[1], x[2]] ...
                            .collect() );
    // fortunately `Tensor` has `min()`
    // `Vec` doesn't have so forced to use `fold` for it
    tdx.min()
}

#[test]
fn test_min_dx() {
    use std::iter::FromIterator;
    use std::cmp::Ordering;

    fn sub<T: Num + Copy>(lhs: &[T], rhs: &[T]) -> Vec<T> {
        Vec::from_iter (lhs.iter().zip(rhs.iter()) .map(|(&x,&y)| x - y ) )
    }

    // 1. prepare non-equidistant vector `x`
    let base_dx = std::f64::consts::FRAC_PI_4; // 0.785398
    let mut x = Vec::from_iter ((1..7) .map(|idx| ((idx as f64).sin() * base_dx).cos().abs() ) );

    x.sort_by(|&x, &y|
        if      x < y { Ordering::Less }
        else if x > y { Ordering::Greater }
        else { Ordering::Equal } );

    let delta_x = sub(&x, &[0.7295467223444416, 0.7556433468867207, 0.7894463883160331,
                            0.8284895327495356, 0.9760168013869176, 0.9938640428738624]);

    let inf_norm = delta_x .iter() .fold(delta_x[0].abs(), |x1, x2| (x1).max((*x2).abs()));

    assert!(inf_norm < 1e-10, "inf_norm");

    // classical way
    let mut min_dx1 = std::f64::MAX;
    for i in 0 .. x.len()-1 {
        min_dx1 = min_dx1.min(x[i+1]-x[i]);
    }

    // functional way
    let min_dx2 = min_dx(&x);
    let abs_delta = (min_dx1 - min_dx2).abs();
    println!("abs_delta : {} min_dx1 : {} min_dx2 : {} x : {:?}", abs_delta, min_dx1, min_dx2, x);
    assert!(abs_delta < 1e-10, "min_dx");
}

fn do_computation(nsteps: usize, ncells: usize, tmax: f64, ifirst: usize, ilast: usize,
                  statelft: f64, statergt: f64, velocity: f64, dt: f64,
                  fc: usize, lc: usize, x: &Tensor<f64>, u: &mut Tensor<f64>)
{
    let mut istep   =   0;
    let mut t       =   0.0f64;
    let mut flux    =   tensor![0.0f64; x.dim(0)];

    // loop over timesteps
    let rbc = u[(ncells+1,)];
    let rslice = tensor![rbc; fc];
    let lslice = tensor![statelft; fc];
    let rindex = [AxisIndex::StridedSlice(Some((ncells+2) as isize), None, 1)];
    let lindex = [AxisIndex::StridedSlice(None, Some(fc as isize), 1)];
    while istep < nsteps && t < tmax {

        // right boundary condition: outgoing wave
        u.index_set( &rindex, &rslice );
        //for ic in ncells .. lc {
        //    u[ic]=u[ncells-1];
        //}
        // left boundary condition: specified value
        u.index_set( &lindex, &lslice );
        //for ic in 0 .. fc {
        //    u[ic]=statelft;
        //}

        // upwind fluxes times dt (ie, flux time integral over cell side)
        // assumes velocity > 0
        let vdt=velocity*dt;
        for ie in ifirst .. ilast+1 {
            flux[(ie,)]=vdt*u[(ie+1,)];
        }

        // conservative difference
        for ic in ifirst .. ilast {
            u[(ic+2,)] -= (flux[(ic+1,)]-flux[(ic,)]) / (x[(ic+1,)]-x[(ic,)])
        }

        // update time and step number
        t       +=  dt;
        istep   +=  1
    }
}

fn main() {
    let mut options = Options {
        verbose:   false,
        num_steps: 2000,
        num_cells: 2000,
        num_runs:  10,
    };

    {
        let mut ap = ArgumentParser::new();
        ap.set_description("Time elapsing of model PDE for scalar conservation law.");
        ap.refer(&mut options.verbose)
            .add_option(&["-v", "--verbose"], StoreTrue,
            "set verbose");
        ap.refer(&mut options.num_steps)
            .add_option(&["-t", "--num_steps"], Store,
            "set number of time steps");
        ap.refer(&mut options.num_cells)
            .add_option(&["-n", "--num_cells"], Store,
            "set number of grid cells");
        ap.refer(&mut options.num_runs)
            .add_option(&["-b", "--num_runs"], Store,
            "set number of runs");
        match ap.parse_args() {
            Ok(()) => {}
            Err(x) => {
                exit(x);
            }
        }
    }

    if options.verbose {
        println!("number of time steps : {} number of grid cells : {} number of runs : {}",
                 options.num_steps, options.num_cells, options.num_runs);
    }

    // problem-specific parameters:
    const jump     : f64   =  0.0f64;
    const x_left   : f64   = -0.2f64;
    const x_right  : f64   =  1.0f64;
    const statelft : f64   =  2.0f64;
    const statergt : f64   =  0.0f64;
    const velocity : f64   =  1.0f64;

    const tmax     : f64   =  0.8f64;
    const cfl      : f64   =  0.9f64;

    // array bounds:
    let   nsteps           = options.num_steps;
    let   ncells           = options.num_cells;
    const fc       : usize = 2;
    let   lc               = ncells+2;
    const ifirst   : usize = 1;
    let   ilast            = ncells-1;

    // work arrays
    // #   double precision
    // #  &  u(-2:ncells+1),
    // #  &  x(0:ncells),
    // #  &  flux(0:ncells)

    let mut u    = tensor![0.0f64; ncells+4];
    let mut x    = tensor![0.0f64; ncells+1];

    //  uniform mesh:
    let dx = (x_right-x_left) / ncells as f64;
    for ie in ifirst .. ilast+1 {
        x[(ie,)] = x_left + ie as f64 * dx;
    }

    // initial values for diffential equation:
    let ijump = cmp::max(ifirst as isize -1,
                         cmp::min((ncells as f64 * (jump-x_left)/(x_right-x_left) + 0.5) as isize,
                                  (ilast+1) as isize)) as usize;
    if options.verbose {
        println!("ijump : {}", ijump);
    }

    // stable timestep (independent of time for linear advection):
    let mindx = min_dx(&x.data());
    let dt = cfl*mindx/velocity.abs();

    //let d = Duration::span(||{
    let timer = std::time::Instant::now();
        for r in 0..options.num_runs {
            if options.verbose {
                println!("run number : {}", r);
            }

            // left state to left of jump
            for ic in ifirst .. ijump-1 {
                u[(ic+2,)] = statelft;
            }

            // volume-weighted average in cell containing jump
            let frac = (jump-x_left-ijump as f64 *dx)/(x_right-x_left);
            u[(ijump+2,)] = statelft*frac+statergt*(1.0f64-frac);

            // right state to right of jump
            for ic in ijump+1 .. ilast {
                u[(ic+2,)]=statergt;
            }

            do_computation(nsteps, ncells, tmax, ifirst, ilast,
                           statelft, statergt, velocity, dt, fc, lc, &x, &mut u);
        }
    //});
    //println!("elapsed time: {:?}s.", timer.elapsed());
    let d = timer.elapsed();
    println!("elapsed time: {:.2}s.", d.as_secs() as f64 + d.subsec_nanos() as f64 / 1.0e9f64);

}
