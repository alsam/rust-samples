extern crate num;
extern crate byteorder;

use std::fs::File;
use std::io::Read;
use std::io::Cursor;
use std::mem;
use byteorder::{ByteOrder, LittleEndian, BigEndian, NativeEndian};
use std::iter::FromIterator;

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

fn main() {
    let mut grid = Grid::<f32>::new();
    grid.read(true, 8, 8, &vec![0u8; 64 * mem::size_of::<f32>()]);

    let mut grid1 = Grid::<c32>::new();
    grid1.read(true, 8, 8, &vec![0u8; 64 * mem::size_of::<c32>()]);
}

