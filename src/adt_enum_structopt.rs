// The MIT License (MIT)
//
// Copyright (c) 2016 Alexander Samoilov
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

// use `structopt` crate instead of `docopt_macros` for command line parsing

#![feature(plugin)]

use structopt::StructOpt;
use std::io;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
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
enum GridType {
    F32T,
    F64T,
    C32T,
    C64T,
}

enum GridVariant {
    GridF32(Box<Grid<f32>>),
    GridF64(Box<Grid<f64>>),
    GridC32(Box<Grid<c32>>),
    GridC64(Box<Grid<c64>>),
}

trait GridTrait<T> {
    // a constructor
    fn new() -> Grid<T> {
        Grid { points: Vec::new() }
    }

    // unformatted read from the buffer
    fn read(&mut self, byte_order: ByteOrder, xsize: usize, ysize: usize, grid_buf: &[u8]);
}

impl GridTrait<f32> for Grid<f32> {
    fn read(&mut self, byte_order: ByteOrder, xsize: usize, ysize: usize, grid_buf: &[u8]) {
        self.points = Vec::from_iter((0..xsize).map(|i| {
            Vec::from_iter((0..ysize).map(|j| {
                let start = (i * ysize + j) * mem::size_of::<f32>();
                let val = read_f32(&grid_buf[start..], byte_order);
                val.unwrap()
            }))
        }));
    }
}

impl GridTrait<c32> for Grid<c32> {
    fn read(&mut self, byte_order: ByteOrder, xsize: usize, ysize: usize, grid_buf: &[u8]) {
        self.points = Vec::from_iter((0..xsize).map(|i| {
            Vec::from_iter((0..ysize).map(|j| {
                let start_1 = (i * ysize + j) * 2 * mem::size_of::<f32>();
                let start_2 = start_1 + mem::size_of::<f32>(); // next f32 number
                let (real, imag) = (read_f32(&grid_buf[start_1..], byte_order),
                                    read_f32(&grid_buf[start_2..], byte_order));
                c32::new(real.unwrap(), imag.unwrap())
            }))
        }));
    }
}

impl GridTrait<f64> for Grid<f64> {
    fn read(&mut self, byte_order: ByteOrder, xsize: usize, ysize: usize, grid_buf: &[u8]) {
        self.points = Vec::from_iter((0..xsize).map(|i| {
            Vec::from_iter((0..ysize).map(|j| {
                let start = (i * ysize + j) * mem::size_of::<f64>();
                let val = read_f64(&grid_buf[start..], byte_order);
                val.unwrap()
            }))
        }));
    }
}

#[derive(StructOpt, Debug)]
#[structopt(name = "gridgen", about = "Grid Generator - an example of Algebraic Data Type (ADT) usage.")]
struct Opt {
    #[structopt(short = "h", long = "help", help = "Print command line parameters")]
    help: bool,

    #[structopt(long = "verbose", help = "Be verbose")]
    verbose: bool,

    #[structopt(short = "d", long = "data-path", help = "path to input data")]
    dpath: String,

    #[structopt(short = "o", long = "out-asy", help = "optional: name of output asy file")]
    asy_file_name: Option<String>,

    /// Needed parameter, the first on the command line.
    #[structopt(help = "Input file")]
    input: String,

}

// the Docopt usage string.
// https://github.com/docopt/docopt.rs
// docopt!(Args derive Debug, "
// Read grid gen.
// 
// Usage:
//   gridgen read <file> [--verbose] [-d <dpath>] [-o <asy-file-name>]
// 
// Options:
//   -h --help       Show this screen.
//   -d --data-path  path to input data.
//   -o --out-asy    name of output asy file.
//   --verbose       Be verbose.
// 
// ");

fn read_from_file(namein: &String,
                  data_path: &str,
                  verbose: bool)
                  -> Result<(ByteOrder, GridType, u32, u32, Vec<u8>), io::Error> {
    // let name = format!("{}/{}.bin", data_path, namein);
    let name = format!("{}/{}", data_path, namein);
    let mut try_open = File::open(name.clone());
    let mut file = match try_open {
        Ok(f) => f,
        Err(err) => panic!("\"{}\" : {}\n", name, err),
    };
    let mut buf = [0u8; 4];
    if verbose {
        println!("read from file: {}", name);
    }
    file.read(&mut buf[0..1])?;
    let is_big_endian = unsafe { mem::transmute::<[u8; 1], u8>([buf[0]]) } == 1;
    let byte_order = if is_big_endian {
        ByteOrder::BigEndian
    } else {
        ByteOrder::LittleEndian
    };

    file.read(&mut buf[..])?;
    let prec = read_u32(&buf[..], byte_order).unwrap();

    file.read(&mut buf[..])?;
    let typ = read_u32(&buf[..], byte_order).unwrap();

    file.read(&mut buf[..])?;
    let xe = read_u32(&buf[..], byte_order).unwrap();

    file.read(&mut buf[..])?;
    let ye = read_u32(&buf[..], byte_order).unwrap();

    if verbose {
        println!("prec: {} (1 - Single Precision, 2 - Double Precision) typ: {} (1 - Real \
                  Values, 2 - Complex Values) xe: {} ye: {}",
                 prec,
                 typ,
                 xe,
                 ye);
    }

    let (grid_type, grid_elem_size) = match (prec, typ) {
        (1, 1) => (GridType::F32T, mem::size_of::<f32>()),
        (1, 2) => (GridType::C32T, mem::size_of::<c32>()),
        (2, 1) => (GridType::F64T, mem::size_of::<f64>()),
        (2, 2) => (GridType::C64T, mem::size_of::<c64>()),
        (_, _) => panic!("illegal combination of prec: {} typ: {}", prec, typ),
    };
    let grid_sz = (xe * ye) as usize;
    let mut grid_buf = Vec::new();
    let sz = file.read_to_end(&mut grid_buf).unwrap();
    if sz != (grid_sz * grid_elem_size) {
        panic!("corrupted grid sz: {} xe*ye: {}", sz, xe * ye);
    }

    Ok((byte_order, grid_type, xe, ye, grid_buf))
}

fn read_grid(namein: &str, dpath: &str, verbose: bool) -> GridVariant {
    let (byte_order, grid_type, xe, ye, grid_buf) =
        read_from_file(&namein.to_string(), &dpath, verbose).unwrap();
    let mask_grid = match grid_type {
        GridType::F32T => {
            let mut f32_grid = Box::new(Grid::<f32>::new());
            (*f32_grid).read(byte_order, xe as usize, ye as usize, &grid_buf);
            GridVariant::GridF32(f32_grid)
        }
        GridType::C32T => {
            let mut c32_grid = Box::new(Grid::<c32>::new());
            (*c32_grid).read(byte_order, xe as usize, ye as usize, &grid_buf);
            GridVariant::GridC32(c32_grid)
        }
        _ => panic!("not yet implemented for grid type: {:?}", grid_type),
    };
    mask_grid
}

fn dump_grid<T: std::fmt::Display>(grid_name: &str, grid: &Grid<T>) {
    println!("-I- dumping grid: {}", grid_name);
    let points = &grid.points;
    for i in 0..points.len() {
        let row = &points[i];
        for j in 0..row.len() {
            let p = &points[i][j]; // as c64;
            println!("{} [{}][{}] = {:}",
                     grid_name,
                     i,
                     j,
                     p /* .to_string() */);
        }
    }
}

fn count_nnz<T: Num>(grid: &Grid<T>) -> (usize, usize) {
    let points = &grid.points;
    let mut nnz = 0;
    for i in 0..points.len() {
        let row = &points[i];
        for j in 0..row.len() {
            let p = &points[i][j];
            if !p.is_zero() {
                nnz += 1;
            }
        }
    }
    (nnz, points.len() * points[0].len())
}

fn count_nnz_functional_way<T: Num>(grid: &Grid<T>) -> (usize, usize) {
    let points = &grid.points;
    let mut nnz = points.iter().fold(0, |sum, row| {
        let incr = {
            row.iter().fold(0, |c, v| if v.is_zero() { c } else { c + 1 })
        };
        sum + incr
    });
    (nnz, points.len() * points[0].len())
}

// bounding box indices for the grid
fn grid_bb_indices<T: Num>(grid: &Grid<T>) -> ((isize, isize), (isize, isize)) {
    let points = &grid.points;
    let ((mut llx, mut lly), (mut urx, mut ury)) = ((std::isize::MAX, std::isize::MAX),
                                                    (std::isize::MIN, std::isize::MIN));
    for i in 0..points.len() {
        let row = &points[i];
        for j in 0..row.len() {
            let p = &points[i][j];
            if !p.is_zero() {
                let ii = i as isize;
                let jj = j as isize;
                if llx > ii {
                    llx = ii;
                }
                if lly > jj {
                    lly = jj;
                }
                if urx < ii {
                    urx = ii;
                }
                if ury < jj {
                    ury = jj;
                }
            }
        }
    }

    ((llx, lly), (urx, ury))
}

fn write_asy<T: Num + std::fmt::Display>(fname: &str,
                                         bounding_box: ((usize, usize), (usize, usize)),
                                         grid: &Grid<T>)
                                         -> Result<(), io::Error> {
    const DELTA: usize = 10;
    let ((llx, lly), (urx, ury)) = bounding_box;
    let mut f = File::create(fname)?;
    write!(f,
           "size({},{});\n",
           (urx - llx) * DELTA,
           (ury - lly) * DELTA)?;
    f.write("
            void draw_grid_cell(pair lb, pair ru, pen p = defaultpen())
            {
              path r = lb -- (ru.x, lb.y) -- ru -- (lb.x, ru.y) -- cycle;
              filldraw(r, p);
            }\n\n"
        .as_bytes())?;

    let points = &grid.points;
    for i in llx..urx {
        for j in lly..ury {
            let p = &points[i][j];
            if !p.is_zero() {
                let lb = (i * DELTA, j * DELTA);
                let ru = ((i + 1) * DELTA, (j + 1) * DELTA);
                write!(f, "draw_grid_cell({:?}, {:?}, gray*{});\n", lb, ru, p)?;
            }
        }
    }

    Ok(())
}


fn main() {
    let opt = Opt::from_args();
    if opt.verbose {
        println!("{:?}", opt);
        println!("input file name: {:?}", opt.input);
    }

    let fname = opt.input;
    let dpath = opt.dpath;
    let asy_fname = if opt.asy_file_name.is_some() {
        opt.asy_file_name.expect("asy file name expected")
    } else {
        format!("{}.asy", fname)
    };
    let (byte_order, grid_type, xe, ye, grid_buf) =
        read_from_file(&fname, &dpath, opt.verbose).expect("failed grid read");

    let mask_grid = read_grid(&fname.clone(), &dpath.clone(), opt.verbose);

    match mask_grid {
        GridVariant::GridF32(agrid) => //dump_grid("f32_grid", &*agrid),
        {
          let (nnz, total) = count_nnz_functional_way(&*agrid);
          println!("nnz for f32 grid: {:} total: {:} ratio: {:.3}%",
                    nnz, total, (nnz as f64 / total as f64)* 100.0);
          let bb = grid_bb_indices(&*agrid);
          let ((llx,lly), (urx,ury)) = bb;
          let (x_extent, y_extent) = (urx-llx, ury-lly);
          println!("a bounding box indices for the grid mask: {:?} x_extent: {:} y_extent: {:}",
                   bb, x_extent, y_extent);
          let halo = 10;
          //write_asy(&asy_fname,
          //          ( ((llx - halo) as usize, (lly - halo) as usize),
          //            ((urx + halo) as usize, (ury + halo) as usize) ),
          //          &*agrid);
          write_asy(&asy_fname,
                    ( ( (llx - halo)         as usize, (lly - halo)         as usize),
                      ( (llx + x_extent / 8) as usize, (lly + y_extent / 8) as usize) ),
                    &*agrid);
        }

        GridVariant::GridC32(agrid) => dump_grid("c32_grid", &*agrid),
        _ => panic!("not yet implemented")
    }


}
