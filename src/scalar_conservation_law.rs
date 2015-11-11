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

fn do_computation(nsteps: usize, ncells: usize, tmax: f64, ifirst: usize, ilast: usize,
                  statelft: f64, statergt: f64, velocity: f64, dt: f64,
                  fc: usize, lc: usize, x: &Vec<f64>, u: &mut Vec<f64>)
{
    let mut istep   =   0;
    let mut t       =   0.0f64;
    let mut flux    =   vec![0.0f64; x.len()];

    // loop over timesteps
    while istep < nsteps && t < tmax {

        // right boundary condition: outgoing wave
        for ic in ncells .. lc {
            u[ic]=u[ncells-1];
        }
        // left boundary condition: specified value
        for ic in 0 .. fc {
            u[ic]=statelft;
        }

        // upwind fluxes times dt (ie, flux time integral over cell side)
        // assumes velocity > 0
        let vdt=velocity*dt;
        for ie in ifirst .. ilast+1 {
            flux[ie]=vdt*u[ie-1];
        }

        // conservative difference
        for ic in ifirst .. ilast {
            u[ic] -= (flux[ic+1]-flux[ic]) / (x[ic+1]-x[ic])
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
    let   lc               = ncells+1;
    const ifirst   : usize = 0;
    let   ilast            = ncells-1;

    // work arrays
    // #   double precision
    // #  &  u(-2:ncells+1),
    // #  &  x(0:ncells),
    // #  &  flux(0:ncells)

    let mut u    = vec![0.0f64; ncells+4];
    let mut x    = vec![0.0f64; ncells+1];

    //  uniform mesh:
    let dx = (x_right-x_left) / ncells as f64;
    for ie in ifirst .. ilast+1 {
        x[ie] = x_left + ie as f64 * dx;
    }

    // initial values for diffential equation:
    let ijump = cmp::max(ifirst as isize -1,
                         cmp::min((ncells as f64 * (jump-x_left)/(x_right-x_left) + 0.5) as isize,
                                  (ilast+1) as isize)) as usize;
    if options.verbose {
        println!("ijump : {}", ijump);
    }

    // stable timestep (independent of time for linear advection):
    let mut mindx = 1.0e300f64;
    for ic in ifirst .. ilast {
        mindx = mindx.min(x[ic+2]-x[ic+1]);
    }
    let dt = cfl*mindx/velocity.abs();

    let d = Duration::span(||{
        for r in 0..options.num_runs {
            if options.verbose {
                println!("run number : {}", r);
            }

            // left state to left of jump
            for ic in ifirst .. ijump-1 {
                u[ic+3] = statelft;
            }

            // volume-weighted average in cell containing jump
            let frac = (jump-x_left-ijump as f64 *dx)/(x_right-x_left);
            u[ijump+3] = statelft*frac+statergt*(1.0f64-frac);

            // right state to right of jump
            for ic in ijump+1 .. ilast {
                u[ic+3]=statergt;
            }

            do_computation(nsteps, ncells, tmax, ifirst, ilast,
                           statelft, statergt, velocity, dt, fc, lc, &x, &mut u);
        }
    });
    println!("elapsed time: {}", d.as_secs() as f64 + d.subsec_nanos() as f64 / 1.0e9f64);
}
