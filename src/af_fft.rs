// https://github.com/arrayfire/arrayfire-rust/issues/51
// a pair of fft(ifft) operations is not identical to original vector. #51 

extern crate arrayfire as af;

use af::*;

fn main() {
    let num_rows: u64 = 4;
    let num_cols: u64 = 1;
    let dims = Dim4::new(&[num_rows, num_cols, 1, 1]);
    let dims1 = Dim4::new(&[1, 1, 1, 1]);
    set_backend(Backend::AF_BACKEND_CPU);
    let a = iota(dims, dims1, Aftype::F32).unwrap();
    println!("a:");
    print(&a);
    let b = fft(&a, 1.0, num_rows as i64).unwrap();
    println!("b:");
    print(&b);
    let c = ifft(&b, 1.0, num_rows as i64).unwrap();
    println!("c:");
    print(&c);
}