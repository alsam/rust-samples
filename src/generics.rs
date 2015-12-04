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

struct Grid<T> {
    points: Vec<Vec<T>>,
}

impl Grid<f32> {
    fn new() -> Self { Grid {points: Vec::new() } }
    fn read(&mut self,
            little_endian: bool,
            xsize: usize,
            ysize: usize,
            grid_buf: &Vec<u8>) {
        self.points = Vec::from_iter((0..xsize).map(|i| {
            Vec::from_iter((0..ysize).map(|j| {
                let start = (i * ysize + j) * mem::size_of::<f32>();
                let real = if little_endian {
                    LittleEndian::read_f32(&grid_buf[start + 0..start + 4])
                } else {
                    BigEndian::read_f32(&grid_buf[start + 0..start + 4])
                };
                real
            }))
        }))
    }
}

impl Grid<c32> {
    fn new() -> Self { Grid {points: Vec::new() } }
    fn read(&mut self,
            little_endian: bool,
            xsize: usize,
            ysize: usize,
            grid_buf: &Vec<u8>) {
         self.points = Vec::from_iter((0..xsize).map(|i| {
            Vec::from_iter((0..ysize).map(|j| {
                let start = (i * ysize + j) * 2 * mem::size_of::<f32>();
                let (real, imag) = if little_endian {
                    (LittleEndian::read_f32(&grid_buf[start + 0..start + 4]),
                     LittleEndian::read_f32(&grid_buf[start + 4..start + 8]))
                } else {
                    (BigEndian::read_f32(&grid_buf[start + 0..start + 4]),
                     BigEndian::read_f32(&grid_buf[start + 4..start + 8]))
                };
                c32::new(real, imag)
            }))
        }));
    }
}

impl Grid<f64> {
    fn new() -> Self { Grid {points: Vec::new() } }
    fn read(&mut self,
            little_endian: bool,
            xsize: usize,
            ysize: usize,
            grid_buf: &Vec<u8>) {
         self.points = Vec::from_iter((0..xsize).map(|i| {
            Vec::from_iter((0..ysize).map(|j| {
                let start = (i * ysize + j) * mem::size_of::<f64>();
                let real = if little_endian {
                    LittleEndian::read_f64(&grid_buf[start + 0..start + 8])
                } else {
                    BigEndian::read_f64(&grid_buf[start + 0..start + 8])
                };
                real
            }))
        }));
    }
}

impl Grid<c64> {
    fn new() -> Self { Grid {points: Vec::new() } }
    fn read(&mut self,
            little_endian: bool,
            xsize: usize,
            ysize: usize,
            grid_buf: &Vec<u8>) {
        self.points =  Vec::from_iter((0..xsize).map(|i| {
            Vec::from_iter((0..ysize).map(|j| {
                let start = (i * ysize + j) * 2 * mem::size_of::<f64>();
                let (real, imag) = if little_endian {
                    (LittleEndian::read_f64(&grid_buf[start + 0..start + 8]),
                     LittleEndian::read_f64(&grid_buf[start + 8..start + 16]))
                } else {
                    (BigEndian::read_f64(&grid_buf[start + 0..start + 8]),
                     BigEndian::read_f64(&grid_buf[start + 8..start + 16]))
                };
                c64::new(real, imag)
            }))
        }));
    }
}

fn main() {
    let mut grid = Grid::<f32>::new();
    grid.read(true, 8, 8, &vec![0u8; 64 * 4]);
}
