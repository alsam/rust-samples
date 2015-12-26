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
    fn new() -> Union {
        Union { data: [0; UNION_SIZE] }
    }

    unsafe fn as_c32_vec(&self) -> *const c32_vec {
        let p = self as *const _ as *const c32_vec;
        p
    }

    unsafe fn as_c64_vec(&self) -> *const c64_vec {
        let p = self as *const _ as *const c64_vec;
        p
    }

    unsafe fn as_c32_vec_mut(&mut self) -> *mut c32_vec {
        let p = self as *mut _ as *mut c32_vec;
        p
    }

    unsafe fn as_c64_vec_mut(&mut self) -> *mut c64_vec {
        let p = self as *mut _ as *mut c64_vec;
        p
    }
}

fn main() {
    println!("Rust union test");

    unsafe {
        let mut union1 = Union::new();
        let mut c32_vec_ptr = union1.as_c32_vec_mut();
        (*c32_vec_ptr).push(c32::new(0.0,1.0));
        (*c32_vec_ptr).push(c32::new(1.0,0.0));
        println!("(*c32_vec_ptr) : {:?}", (*c32_vec_ptr));

        let mut union2 = Union::new();
        let mut c64_vec_ptr = union2.as_c64_vec_mut();
        (*c64_vec_ptr).push(c64::new(0.0,1.0));
        (*c64_vec_ptr).push(c64::new(1.0,0.0));
        println!("(*c64_vec_ptr) : {:?}", (*c64_vec_ptr));

    }
}
