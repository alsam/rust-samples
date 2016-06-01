extern crate num;
extern crate byteorder;
extern crate libc;

use std::env;
use std::fs::File;
use std::io::{Read, Write};
use std::io::Cursor;
use std::mem;
use std::iter::FromIterator;
use byteorder::{ByteOrder, LittleEndian, BigEndian, NativeEndian};
use std::time::Duration;

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

fn kernel1(ai: &mut Vec<f32>, ef: &Vec<c32>) {
    let size = ai.len();
    for i in 0..size {
        ai[i] += ef[i].re.powi(2) + ef[i].im.powi(2);
    }
}

fn kernel2(ai: &mut Vec<f32>, ef_re: &Vec<f32>, ef_im: &Vec<f32>) {
    let size = ai.len();
    for i in 0..size {
        ai[i] += ef_re[i].powi(2) + ef_im[i].powi(2);
    }
}

fn kernel3(ai: &mut Vec<f32>, ef: &Vec<f32>) {
    let size = ai.len();
    for i in 0..size {
        ai[i] += ef[2*i].powi(2) + ef[2*i+1].powi(2);
    }
}

// now play with kernels written in C/C++
#[link(name = "ckernels")]
extern "C" {
    fn kernel4(L: c_int, ai: *mut f32, ef: *const f32);
    fn kernel5(L: c_int, ai: *mut f32, ef: *const f32);
}

fn main() {
    let arg = env::args_os().nth(1).expect("Please, provide a file as argument");
    let mut file = File::open(&arg).ok().expect(&format!("The file {:?} does not exist", arg));
    let file_info = file.metadata().ok().expect("Cannot get file metadata");
    if !file_info.is_file() {
        panic!("{:?} is not a file", arg);
    }
    println!("the file {:?} has length {} bytes", arg, file_info.len());
    let mut buf = [0u8; 4];
    file.read(&mut buf[..]);
    let L = LittleEndian::read_u32(&buf[..]);
    println!("L : {}", L);
    let size = L as usize;
    let mut grid_buf = Vec::new();
    let sz = file.read_to_end(&mut grid_buf).unwrap();
    let mut ai = Vec::from_iter((0..size).map(|i| {
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


    let ef_re = Vec::from_iter((0..size).map(|i| ef[i].re));
    let ef_im = Vec::from_iter((0..size).map(|i| ef[i].im));
    let ef_as_f32 = Vec::from_iter((0..2*size).map(|i| {
                let j = i/2;
                if i%2 == 0 {ef[j].re} else {ef[j].im}
            }));

    let rep_count = 10000;
    let mut timer = timer_start!();

    for i in 0..rep_count {
        kernel1(&mut ai, &ef);
    }

    timer_stop!(timer, "kernel1");
    timer = timer_start!();

    for i in 0..rep_count {
        kernel2(&mut ai, &ef_re, &ef_im);
    }

    timer_stop!(timer, "kernel2");
    timer = timer_start!();

    for i in 0..rep_count {
        kernel3(&mut ai, &ef_as_f32);
    }

    timer_stop!(timer, "kernel3");


    let len = size as c_int;
    let pai = ai.as_mut_ptr();
    let pef = ef_as_f32.as_ptr();
 
    timer = timer_start!();

    for i in 0..rep_count {
        unsafe {
            kernel4(len, pai, pef);
        }
    }

    timer_stop!(timer, "kernel4");
    timer = timer_start!();

    for i in 0..rep_count {
        unsafe {
            kernel5(len, pai, pef);
        }
    }

    timer_stop!(timer, "kernel5");


    println!("fini");
}
