extern crate num;
extern crate byteorder;

use std::env;
use std::fs::File;
use std::io::{Read, Write};
use std::io::Cursor;
use std::mem;
use std::iter::FromIterator;
use byteorder::{ByteOrder, LittleEndian, BigEndian, NativeEndian};
use std::time::Duration;

type c32 = num::Complex<f32>;

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

    let rep_count = 10000;
    let mut timer = std::time::Instant::now();

    for i in 0..rep_count {
      kernel1(&mut ai, &ef);
    }

    let d = timer.elapsed();
    println!("elapsed time for kernel1: {:.7}s.", d.as_secs() as f64 + d.subsec_nanos() as f64 / 1.0e9f64);

    timer = std::time::Instant::now();

    for i in 0..rep_count {
      kernel2(&mut ai, &ef_re, &ef_im);
    }

    let d = timer.elapsed();
    println!("elapsed time for kernel2: {:.7}s.", d.as_secs() as f64 + d.subsec_nanos() as f64 / 1.0e9f64);

    println!("fini");
}
