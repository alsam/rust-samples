//***********************************************************************
//  Copyright 2006 John A. Trangenstein
//
//  This software is made available for research and instructional use 
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
use std::process::exit;
use argparse::{ArgumentParser, StoreTrue, Store};

struct Options {
    verbose: bool,
    num_steps: usize,
    num_cells: usize,
    num_runs: usize,
}

fn do_computation(nsteps: usize, ncells: usize)
{
}

fn main() {
    let mut options = Options {
        verbose: false,
        num_steps: 2000,
        num_cells: 2000,
        num_runs: 10,
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

    let d = Duration::span(||{
        for r in 0..options.num_runs {
            if options.verbose {
                println!("run number : {}", r);
            }

            do_computation(options.num_steps, options.num_cells);
        }
    });
    println!("elapsed time: {}", d.as_secs() as f64 + d.subsec_nanos()as f64 / 1000000000.0f64);
}
