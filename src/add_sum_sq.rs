#![feature(test)]
extern crate test;

extern crate num;
extern crate byteorder;
extern crate libc;
extern crate argparse;

use test::Bencher;
use std::env;
use std::fs::File;
use std::io::{Read, Write};
use std::io::Cursor;
use std::mem;
use std::iter::FromIterator;
use byteorder::{ByteOrder, LittleEndian, BigEndian, NativeEndian};
use std::time::Duration;
use argparse::{ArgumentParser, StoreTrue, Store};
use std::process::exit;

use libc::{c_int, size_t};
use std::vec;

type c32 = num::Complex<f32>;


macro_rules! timer_start {
    () => {
        {
            let timer = std::time::Instant::now();
            timer
        }
    }
}

macro_rules! timer_stop {
    ($timer:expr, $name:expr) => {
        let d = ($timer).elapsed();
        println!("elapsed time for {}: {:.7}s.", $name, d.as_secs() as f64 + d.subsec_nanos() as f64 / 1.0e9f64);
    }
}

fn kernel1(ai: &mut [f32], ef: &[c32]) {
    let size = ai.len();
    for i in 0..size {
        ai[i] += ef[i].re.powi(2) + ef[i].im.powi(2);
    }
}

fn kernel2(ai: &mut [f32], ef_re: &[f32], ef_im: &[f32]) {
    let size = ai.len();
    for i in 0..size {
        ai[i] += ef_re[i].powi(2) + ef_im[i].powi(2);
    }
}

fn kernel3(ai: &mut [f32], ef: &[f32]) {
    let size = ai.len();
    for i in 0..size {
        ai[i] += ef[2*i].powi(2) + ef[2*i+1].powi(2);
    }
}

// now play with kernels written in C/C++
#[link(name = "ckernels-add-sum-sq")]
extern "C" {
    fn kernel4(L: c_int, ai: *mut f32, ef: *const f32);
    fn kernel5(L: c_int, ai: *mut f32, ef: *const f32);
    fn kernel6(L: c_int, ai: *mut f32, ef: *const f32);
    fn kernel7(L: c_int, ai: *mut f32, ef: *const f32);
}

fn kernel8(ai: &[f32], ef: &[c32]) -> Vec<f32> {
    let x: Vec<f32> = ai.iter()
                .zip( ef.iter() )
                .map(
                    |(x, y)| x + y.re.powi(2) + y.im.powi(2)
                    ).collect();
    x
}

fn kernel9(ai: &mut [f32], ef: &[c32]) {
    for (x, y) in ai.iter_mut().zip(ef) {
        *x += (*y).re.powi(2) + (*y).im.powi(2);
    }
}

struct Options {
    verbose: bool,
    name: String,
    rep_count: usize,
    kernel_num: isize,
}

fn cook_input_data(parse_options: bool) -> (Options, Vec<f32>, Vec<c32>) {
    let mut options = Options {
        verbose:   false,
        //name:      "./data/add_sum_sq/sum_ef_20077.bin".to_string(),
        name:      "./data/add_sum_sq/sum_ef_20085.bin".to_string(),
        rep_count: 10000,
        kernel_num: -1,
    };

    if parse_options {
        let mut ap = ArgumentParser::new();
        ap.set_description("Time elapsing for add_sum_sq kernels.");
        ap.refer(&mut options.verbose)
            .add_option(&["-v", "--verbose"], StoreTrue,
            "set verbose");
        ap.refer(&mut options.name)
            .add_option(&["-i", "--input"], Store,
            "set name of input file");
        ap.refer(&mut options.rep_count)
            .add_option(&["-n", "--num_iters"], Store,
            "set number of iterations");
        ap.refer(&mut options.kernel_num)
            .add_option(&["-k", "--num_kernel"], Store,
            "set number of kernel");
        match ap.parse_args() {
            Ok(()) => {}
            Err(x) => {
                exit(x);
            }
        }
    }

    if options.verbose {
        println!("input file name : {} number of iterations : {} kernel number : {}",
                 options.name, options.rep_count, options.kernel_num);
    }

    let mut file = File::open(&options.name).ok().expect(&format!("The file {:?} does not exist", &options.name));
    let file_info = file.metadata().ok().expect("Cannot get file metadata");
    if !file_info.is_file() {
        panic!("{:?} is not a file", &options.name);
    }

    if parse_options {
        println!("the file {:?} has length {} bytes", &options.name, file_info.len());
    }

    let mut buf = [0u8; 4];
    file.read(&mut buf[..]);
    let L = LittleEndian::read_u32(&buf[..]);

    if parse_options {
        println!("L : {}", L);
    }

    let size = L as usize;
    let mut grid_buf = Vec::new();
    let sz = file.read_to_end(&mut grid_buf).unwrap();
    let ai = Vec::from_iter((0..size).map(|i| {
                let start = i * mem::size_of::<f32>();
                let val = LittleEndian::read_f32(&grid_buf[start .. ]);
                //println!("val : {}",val);
                val
            }));
    let ef = Vec::from_iter((0..size).map(|i| {
                let offset = size * mem::size_of::<f32>();
                let start = offset + i * mem::size_of::<c32>();
                let val_real = LittleEndian::read_f32(&grid_buf[start                         .. ]);
                let val_imag = LittleEndian::read_f32(&grid_buf[start + mem::size_of::<f32>() .. ]);
                //println!("val_real : {}  val_imag : {}",val_real,val_imag);
                c32::new(val_real, val_imag)
            }));

    (options, ai, ef)
}

fn ef_views(ef: &[c32]) -> (Vec<f32>, Vec<f32>, Vec<f32>) {
    let size = ef.len();
    let ef_re = Vec::from_iter((0..size).map(|i| ef[i].re));
    let ef_im = Vec::from_iter((0..size).map(|i| ef[i].im));
    let ef_as_f32 = Vec::from_iter((0..2*size).map(|i| {
                let j = i/2;
                if i%2 == 0 {ef[j].re} else {ef[j].im}
            }));
    (ef_re, ef_im, ef_as_f32)
}

fn check_kernels_diff(ai: &Vec<f32>, ef_as_f32: &Vec<f32>) -> Vec<f32> {
    let mut ai_copy1 = ai.clone();
    let mut ai_copy2 = ai.clone();
    let size      = ai.len();
    let len       = size as c_int;
    let pai_copy1 = ai_copy1.as_mut_ptr();
    let pai_copy2 = ai_copy2.as_mut_ptr();
    let pef       = ef_as_f32.as_ptr();

    unsafe {
            kernel5(len, pai_copy1, pef);
            kernel7(len, pai_copy2, pef);
    }

    let x: Vec<f32> = ai_copy1.iter()
                .zip( ai_copy2.iter() )
                .map(
                    |(x, y)| (x - y)
                    ).collect();

    x
}

fn main() {

    let (options, mut ai, ef) = cook_input_data(true);
    let size = ai.len();

    let (ef_re, ef_im, ef_as_f32) = ef_views(&ef);

    let rep_count = options.rep_count;

    if options.kernel_num == 9 || options.kernel_num == -1 {
        let timer = timer_start!();
  
        for _ in 0..rep_count {
            kernel9(&mut ai, &ef);
        }
  
        timer_stop!(timer, "kernel9");
    }

    if options.kernel_num == 1 || options.kernel_num == -1 {
        let timer = timer_start!();
  
        for _ in 0..rep_count {
            kernel1(&mut ai, &ef);
        }
  
        timer_stop!(timer, "kernel1");
    }

    if options.kernel_num == 2 || options.kernel_num == -1 {
        let timer = timer_start!();

        for _ in 0..rep_count {
            kernel2(&mut ai, &ef_re, &ef_im);
        }

        timer_stop!(timer, "kernel2");
    }

    if options.kernel_num == 3 || options.kernel_num == -1 {
        let timer = timer_start!();

        for _ in 0..rep_count {
            kernel3(&mut ai, &ef_as_f32);
        }

        timer_stop!(timer, "kernel3");
    }

    if options.kernel_num == 4 || options.kernel_num == -1 {
        let len = size as c_int;
        let pai = ai.as_mut_ptr();
        let pef = ef_as_f32.as_ptr();
        let timer = timer_start!();
    
        for _ in 0..rep_count {
            unsafe {
                kernel4(len, pai, pef);
            }
        }
    
        timer_stop!(timer, "kernel4");
    }

    if options.kernel_num == 5 || options.kernel_num == -1 {
        let len = size as c_int;
        let pai = ai.as_mut_ptr();
        let pef = ef_as_f32.as_ptr();
        let timer = timer_start!();

        for _ in 0..rep_count {
            unsafe {
                kernel5(len, pai, pef);
            }
        }

        timer_stop!(timer, "kernel5");
        if rep_count == 1 {
            println!("output from kernel 5 ai: {:?}", ai);
        }

    }


    if options.kernel_num == 7 || options.kernel_num == -1 {
        let len = size as c_int;
        let pai = ai.as_mut_ptr();
        let pef = ef_as_f32.as_ptr();
        let timer = timer_start!();

        for _ in 0..rep_count {
            unsafe {
                kernel7(len, pai, pef);
            }
        }

        timer_stop!(timer, "kernel7");
        if rep_count == 1 {
            println!("output from kernel 7 ai: {:?}", ai);
        }
    }

    let diff = check_kernels_diff(&ai, &ef_as_f32);
    let mut ind = 0;
    let mut max_diff = 0.0;
    for (i, val) in diff.into_iter().enumerate() {
        println!("{} {:.17}", i, val);
        if val.abs() > max_diff {
            max_diff = val.abs();
            ind = i;
        }
    }
    println!("max_diff : {:.17} ind : {}", max_diff, ind);
    //println!("diff : {:?}",diff);

    println!("fini");
}


#[bench]
fn setup_kernel1(b: &mut Bencher) {
    let (_, mut ai, ef) = cook_input_data(false);

    b.iter(|| {
        kernel1(&mut ai, &ef);
    } )
}

#[bench]
fn setup_kernel2(b: &mut Bencher) {
    let (_, mut ai, ef) = cook_input_data(false);
    let (ef_re, ef_im, _) = ef_views(&ef);

    b.iter(|| {
        kernel2(&mut ai, &ef_re, &ef_im);
    } )
}

#[bench]
fn setup_kernel3(b: &mut Bencher) {
    let (_, mut ai, ef) = cook_input_data(false);
    let (_, _, ef_as_f32) = ef_views(&ef);

    b.iter(|| {
        kernel3(&mut ai, &ef_as_f32);
    } )
}

#[bench]
fn setup_kernel4(b: &mut Bencher) {
    let (_, mut ai, ef) = cook_input_data(false);
    let size = ai.len();

    let (ef_re, ef_im, ef_as_f32) = ef_views(&ef);

    let len = size as c_int;
    let pai = ai.as_mut_ptr();
    let pef = ef_as_f32.as_ptr();

    b.iter(|| {
        unsafe {
            kernel4(len, pai, pef);
        }
    } )
}

#[bench]
fn setup_kernel5(b: &mut Bencher) {
    let (_, mut ai, ef) = cook_input_data(false);
    let size = ai.len();

    let (ef_re, ef_im, ef_as_f32) = ef_views(&ef);

    let len = size as c_int;
    let pai = ai.as_mut_ptr();
    let pef = ef_as_f32.as_ptr();

    b.iter(|| {
        unsafe {
            kernel5(len, pai, pef);
        }
    } )
}

#[bench]
fn setup_kernel6(b: &mut Bencher) {
    let (_, mut ai, ef) = cook_input_data(false);
    let size = ai.len();

    let (ef_re, ef_im, ef_as_f32) = ef_views(&ef);

    let len = size as c_int;
    let pai = ai.as_mut_ptr();
    let pef = ef_as_f32.as_ptr();

    b.iter(|| {
        unsafe {
            kernel6(len, pai, pef);
        }
    } )
}

#[bench]
fn setup_kernel7(b: &mut Bencher) {
    let (_, mut ai, ef) = cook_input_data(false);
    let size = ai.len();

    let (ef_re, ef_im, ef_as_f32) = ef_views(&ef);

    let len = size as c_int;
    let pai = ai.as_mut_ptr();
    let pef = ef_as_f32.as_ptr();

    b.iter(|| {
        unsafe {
            kernel7(len, pai, pef);
        }
    } )
}


#[bench]
fn setup_kernel8(b: &mut Bencher) {
    let (_, mut ai, ef) = cook_input_data(false);

    b.iter(|| {
        ai = kernel8(&ai, &ef);
    } )
}

#[bench]
fn setup_kernel9(b: &mut Bencher) {
    let (_, mut ai, ef) = cook_input_data(false);

    b.iter(|| {
        kernel9(&mut ai, &ef);
    } )
}


