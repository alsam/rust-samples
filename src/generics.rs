extern crate num;
extern crate byteorder;

use std::fs::File;
use std::io::Read;
use std::io::Cursor;
use std::mem;
use byteorder::{ByteOrder, LittleEndian, BigEndian, NativeEndian};
use std::iter::FromIterator;
use num::Num;

type c32 = num::Complex<f32>;
type c64 = num::Complex<f64>;

trait GridTrait<T> {
    // a constructor
    fn new() -> Grid<T> { Grid {points: Vec::new() } }

    // unformatted read from the buffer
    fn read(&mut self,
            little_endian: bool,
            xsize: usize,
            ysize: usize,
            grid_buf: &Vec<u8>);
}

struct Grid<T> {
    points: Vec<Vec<T>>,
}

//impl <T> Grid<T> {
//    fn new() -> Grid<T> { panic!("not implemented for generic type"); }
//}

impl GridTrait<f32> for Grid<f32> {
    fn read(&mut self,
            little_endian: bool,
            xsize: usize,
            ysize: usize,
            grid_buf: &Vec<u8>) {
        self.points = Vec::from_iter((0..xsize).map(|i| {
            Vec::from_iter((0..ysize).map(|j| {
                let start = (i * ysize + j) * mem::size_of::<f32>();
                let real = if little_endian {
                    LittleEndian::read_f32(&grid_buf[start .. ])
                } else {
                    BigEndian::read_f32(&grid_buf[start .. ])
                };
                real
            }))
        }))
    }
}

impl GridTrait<c32> for Grid<c32> {
    fn read(&mut self,
            little_endian: bool,
            xsize: usize,
            ysize: usize,
            grid_buf: &Vec<u8>) {
         self.points = Vec::from_iter((0..xsize).map(|i| {
            Vec::from_iter((0..ysize).map(|j| {
                let start_1 = (i * ysize + j) * 2 * mem::size_of::<f32>();
                let start_2 = start_1 + mem::size_of::<f32>(); // next f32 number
                let (real, imag) = if little_endian {
                    (LittleEndian::read_f32(&grid_buf[start_1 .. ]),
                     LittleEndian::read_f32(&grid_buf[start_2 .. ]))
                } else {
                    (BigEndian::read_f32(&grid_buf[start_1 .. ]),
                     BigEndian::read_f32(&grid_buf[start_2 .. ]))
                };
                c32::new(real, imag)
            }))
        }));
    }
}

impl GridTrait<f64> for Grid<f64> {
    fn read(&mut self,
            little_endian: bool,
            xsize: usize,
            ysize: usize,
            grid_buf: &Vec<u8>) {
         self.points = Vec::from_iter((0..xsize).map(|i| {
            Vec::from_iter((0..ysize).map(|j| {
                let start = (i * ysize + j) * mem::size_of::<f64>();
                let real = if little_endian {
                    LittleEndian::read_f64(&grid_buf[start .. ])
                } else {
                    BigEndian::read_f64(&grid_buf[start .. ])
                };
                real
            }))
        }));
    }
}

impl GridTrait<c64> for Grid<c64> {
    fn read(&mut self,
            little_endian: bool,
            xsize: usize,
            ysize: usize,
            grid_buf: &Vec<u8>) {
        self.points =  Vec::from_iter((0..xsize).map(|i| {
            Vec::from_iter((0..ysize).map(|j| {
                let start_1 = (i * ysize + j) * 2 * mem::size_of::<f64>();
                let start_2 = start_1 + mem::size_of::<f64>(); // next f64 number
                let (real, imag) = if little_endian {
                    (LittleEndian::read_f64(&grid_buf[start_1 .. ]),
                     LittleEndian::read_f64(&grid_buf[start_2 .. ]))
                } else {
                    (BigEndian::read_f64(&grid_buf[start_1 .. ]),
                     BigEndian::read_f64(&grid_buf[start_2 .. ]))
                };
                c64::new(real, imag)
            }))
        }));
    }
}

fn grid_dims<T>(grid: &Grid<T>) -> (usize, usize) {
    let points = &grid.points;
    (points.len(), points[0].len())
}

fn dump_grid<T: std::fmt::Display>(grid_name: &str, grid: &Grid<T>) {
    println!("-I- dumping grid: {}", grid_name);
    let points = &grid.points;
    for i in 0 .. points.len() {
        let row = &points[i];
        for j in 0 .. row.len() {
            let p = &points[i][j];
            println!("{} [{}][{}] = {:}", grid_name, i, j, p /*.to_string()*/);
        }
    }
}

fn main() {
    let mut grid = Grid::<f32>::new();
    grid.read(true, 8, 8, &vec![77u8; 64 * mem::size_of::<f32>()]);

    let mut grid1 = Grid::<c32>::new();
    grid1.read(true, 8, 8, &vec![121u8; 64 * mem::size_of::<c32>()]);
    println!("dims for grid1: {:?}", grid_dims(&grid1));

    //let mut grid2 = Grid::<i32>::new();

    dump_grid("grid",  &grid);
    dump_grid("grid1", &grid1);
}

