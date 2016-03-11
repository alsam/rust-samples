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

// Algebraic Data Type (ADT) `GridVariant` use case using Rust `enum`
// should be more robust than `std::Any`

#![feature(plugin)]
#![plugin(docopt_macros)]
#![feature(custom_derive)]

extern crate rustc_serialize;
extern crate docopt;
extern crate num;

use docopt::Docopt;
use std::io;
use std::fs::File;
use std::io::Read;
use std::io::Cursor;
use std::mem;
use std::iter::FromIterator;

extern crate endianness;
use endianness::{ByteOrder, read_u32, read_f32, read_f64};
use num::Num;

use std::fmt::Debug;

type c32 = num::Complex<f32>;
type c64 = num::Complex<f64>;


struct Grid<T> {
    points: Vec<Vec<T>>,
}

#[derive(Debug)]
enum GridType { f32_t, f64_t, c32_t, c64_t, }

enum GridVariant {
    Grid_f32(Box<Grid<f32>>),
    Grid_f64(Box<Grid<f64>>),
    Grid_c32(Box<Grid<c32>>),
    Grid_c64(Box<Grid<c64>>),
}

trait GridTrait<T> {
    // a constructor
    fn new() -> Grid<T> { Grid {points: Vec::new() } }

    // unformatted read from the buffer
    fn read(&mut self,
            byte_order: ByteOrder,
            xsize: usize,
            ysize: usize,
            grid_buf: &Vec<u8>);
}

impl GridTrait<f32> for Grid<f32> {
    fn read(&mut self,
            byte_order: ByteOrder,
            xsize: usize,
            ysize: usize,
            grid_buf: &Vec<u8>) {
        self.points = Vec::from_iter((0..xsize).map(|i| {
            Vec::from_iter((0..ysize).map(|j| {
                let start = (i * ysize + j) * mem::size_of::<f32>();
                let val = read_f32(&grid_buf[start .. ], byte_order);
                val.unwrap()
            }))
        }));
    }
}

impl GridTrait<c32> for Grid<c32> {
    fn read(&mut self,
            byte_order: ByteOrder,
            xsize: usize,
            ysize: usize,
            grid_buf: &Vec<u8>) {
         self.points = Vec::from_iter((0..xsize).map(|i| {
            Vec::from_iter((0..ysize).map(|j| {
                let start_1 = (i * ysize + j) * 2 * mem::size_of::<f32>();
                let start_2 = start_1 + mem::size_of::<f32>(); // next f32 number
                let (real, imag) =
                    (read_f32(&grid_buf[start_1 .. ], byte_order),
                     read_f32(&grid_buf[start_2 .. ], byte_order));
                c32::new(real.unwrap(), imag.unwrap())
            }))
        }));
    }
}

impl GridTrait<f64> for Grid<f64> {
    fn read(&mut self,
            byte_order: ByteOrder,
            xsize: usize,
            ysize: usize,
            grid_buf: &Vec<u8>) {
         self.points = Vec::from_iter((0..xsize).map(|i| {
            Vec::from_iter((0..ysize).map(|j| {
                let start = (i * ysize + j) * mem::size_of::<f64>();
                let val = read_f64(&grid_buf[start .. ], byte_order);
                val.unwrap()
            }))
        }));
    }
}

// the Docopt usage string.
// https://github.com/docopt/docopt.rs
docopt!(Args derive Debug, "
Read grid gen.

Usage:
  gridgen read <file> [--verbose] [-d <dpath>]

Options:
  -h --help       Show this screen.
  -d --data-path  path to input data.
  --verbose       Be verbose.

");

fn read_from_file(namein: &String, data_path: &String, verbose: bool) -> Result<(ByteOrder, GridType, u32, u32, Vec<u8>), io::Error> {
    //let name = format!("{}/{}.bin", data_path, namein);
    let name = format!("{}/{}", data_path, namein);
    let mut try_open = File::open(name.clone());
    let mut file = match try_open {
        Ok(f) => f,
        Err(err) => {panic!("\"{}\" : {}\n",name, err)}
    };
    let mut buf = [0u8; 4];
    if verbose {
        println!("read from file: {}", name);
    }
    try!(file.read(&mut buf[0..1]));
    let is_big_endian = unsafe {mem::transmute::<[u8; 1], u8>([buf[0]])} == 1;
    let byte_order = if is_big_endian {ByteOrder::BigEndian} else {ByteOrder::LittleEndian};

    try!(file.read(&mut buf[..]));                
    let prec = read_u32(&buf[..], byte_order).unwrap();

    try!(file.read(&mut buf[..]));                
    let typ = read_u32(&buf[..], byte_order).unwrap();

    try!(file.read(&mut buf[..]));                
    let xe = read_u32(&buf[..], byte_order).unwrap();

    try!(file.read(&mut buf[..]));                
    let ye = read_u32(&buf[..], byte_order).unwrap();

    if verbose {
        println!("prec: {} (1 - Single Precision, 2 - Double Precision) typ: {} (1 - Real Values, 2 - Complex Values) xe: {} ye: {}",
                 prec, typ, xe, ye);
    }

    let (grid_type, grid_elem_size) =
        match (prec,typ) {
            (1,1) => (GridType::f32_t, mem::size_of::<f32>()),
            (1,2) => (GridType::c32_t, mem::size_of::<c32>()),
            (2,1) => (GridType::f64_t, mem::size_of::<f64>()),
            (2,2) => (GridType::c64_t, mem::size_of::<c64>()),
            (_,_) => panic!("illegal combination of prec: {} typ: {}", prec, typ)
        };
    let grid_sz = (xe * ye) as usize;
    let mut grid_buf = Vec::new();
    let sz = file.read_to_end(&mut grid_buf).unwrap();
    if sz != (grid_sz * grid_elem_size) {
        panic!("corrupted grid sz: {} xe*ye: {}", sz, xe * ye);
    }

    Ok((byte_order, grid_type, xe, ye, grid_buf))
}

fn dump_grid<T: std::fmt::Display>(grid_name: &str, grid: &Grid<T>) {
    println!("-I- dumping grid: {}", grid_name);
    let points = &grid.points;
    for i in 0 .. points.len() {
        let row = &points[i];
        for j in 0 .. row.len() {
            let p = &points[i][j] ; // as c64;
            println!("{} [{}][{}] = {:}", grid_name, i, j, p /*.to_string()*/);
        }
    }
}


fn main() {
    let args: Args = Args::docopt().decode().unwrap_or_else(|e| e.exit());

    if args.flag_verbose {
        println!("{:?}", args);
        println!("input file name: {:?}", args.arg_file);
    }

    let fname = args.arg_file;
    let dpath = args.arg_dpath;
    let (byte_order, grid_type, xe, ye, grid_buf) = read_from_file(&fname, &dpath, args.flag_verbose).unwrap();

    let mut mask_grid =
        match grid_type {
            GridType::f32_t => {
                let mut f32_grid = Box::new(Grid::<f32>::new());
                (*f32_grid).read(byte_order, xe as usize, ye as usize, &grid_buf);
                GridVariant::Grid_f32(f32_grid)
            },
            GridType::c32_t => {
                let mut c32_grid = Box::new(Grid::<c32>::new());
                (*c32_grid).read(byte_order, xe as usize, ye as usize, &grid_buf);
                GridVariant::Grid_c32(c32_grid)
            },
            _ => panic!("not yet implemented for grid type: {:?}", grid_type)
        };

    match mask_grid {
        GridVariant::Grid_f32(agrid) => dump_grid("f32_grid", &*agrid),
        GridVariant::Grid_c32(agrid) => dump_grid("c32_grid", &*agrid),
        _ => panic!("not yet implemented")
    }


}
