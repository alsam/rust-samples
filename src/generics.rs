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

// [Working around the lack of associated method on parametric traits?](http://stackoverflow.com/questions/27888069/working-around-the-lack-of-associated-method-on-parametric-traits)

trait ReadGrid<T> {
    fn read(_unused: Option<Self>,
            little_endian: bool,
            xsize: usize,
            ysize: usize,
            grid_buf: &Vec<u8>)
            -> Vec<Vec<T>>;
}

impl ReadGrid<f32> for f32 {
    fn read(_unused: Option<Self>,
            little_endian: bool,
            xsize: usize,
            ysize: usize,
            grid_buf: &Vec<u8>)
            -> Vec<Vec<f32>> {
        Vec::from_iter((0..xsize).map(|i| {
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

impl ReadGrid<c32> for c32 {
    fn read(_unused: Option<Self>,
            little_endian: bool,
            xsize: usize,
            ysize: usize,
            grid_buf: &Vec<u8>)
            -> Vec<Vec<c32>> {
        Vec::from_iter((0..xsize).map(|i| {
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
        }))
    }
}

impl ReadGrid<f64> for f64 {
    fn read(_unused: Option<Self>,
            little_endian: bool,
            xsize: usize,
            ysize: usize,
            grid_buf: &Vec<u8>)
            -> Vec<Vec<f64>> {
        Vec::from_iter((0..xsize).map(|i| {
            Vec::from_iter((0..ysize).map(|j| {
                let start = (i * ysize + j) * mem::size_of::<f64>();
                let real = if little_endian {
                    LittleEndian::read_f64(&grid_buf[start + 0..start + 8])
                } else {
                    BigEndian::read_f64(&grid_buf[start + 0..start + 8])
                };
                real
            }))
        }))
    }
}

impl ReadGrid<c64> for c64 {
    fn read(_unused: Option<Self>,
            little_endian: bool,
            xsize: usize,
            ysize: usize,
            grid_buf: &Vec<u8>)
            -> Vec<Vec<c64>> {
        Vec::from_iter((0..xsize).map(|i| {
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
        }))
    }
}

struct Grid<T> {
    points: Vec<Vec<T>>,
}



fn main() {
    let mut grid = ReadGrid::read(None::<f32>, true, 16, 8, &vec![0u8; 64 * 4]);
}
