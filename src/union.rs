extern crate num;

use std::mem;

type c32 = num::Complex<f32>;
type c64 = num::Complex<f64>;

const UNION_SIZE : usize = 8; //mem::size_of::<usize>();

#[repr(C)]
struct Union {
    data: [usize; UNION_SIZE]
}

type c32_vec = Vec<c32>;
type c64_vec = Vec<c64>;

impl Union {
    unsafe fn as_c32_vec(&self) -> *const c32_vec {
        let p = self as *const _ as *const c32_vec;
        p
    }

    unsafe fn as_c64_vec(&self) -> *const c64_vec {
        let p = self as *const _ as *const c64_vec;
        p
    }

    //unsafe fn as_c32_vec_mut(&self) -> *mut c32_vec {
    //    let p = self as *mut _ as *mut c32_vec;
    //    &mut p
    //}

    //unsafe fn as_c64_vec_mut(&self) -> *mut c64_vec {
    //    let p = self as *mut _ as *mut c64_vec;
    //    &mut p
    //}


}

fn main() {
    println!("Rust union test");


}
