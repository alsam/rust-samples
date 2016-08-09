// https://github.com/arrayfire/arrayfire-rust/issues/51
// a pair of fft(ifft) operations is not identical to original vector. #51 

// Aftype -> DType [compilation errors with sample code](https://github.com/arrayfire/arrayfire-rust/issues/75)

extern crate arrayfire as af;

use af::*;

fn test_backend() {
    let num_rows: u64 = 4;
    let num_cols: u64 = 1;
    let dims = Dim4::new(&[num_rows, num_cols, 1, 1]);
    let dims1 = Dim4::new(&[1, 1, 1, 1]);
    let a = iota::<f32>(dims, dims1);
    println!("a:");
    print(&a);
    let b = fft(&a, 1.0, num_rows as i64);
    println!("b:");
    print(&b);
    let c = ifft(&b, 1.0, num_rows as i64);
    println!("c:");
    print(&c);
}

fn main() {
    let available = get_available_backends();
    if available.contains(&Backend::CPU) {
        println!("Evaluating CPU Backend...");
        set_backend(Backend::CPU);
        println!("There are {} CPU compute devices", device_count());
        test_backend();
    }

    if available.contains(&Backend::CUDA) {
        println!("Evaluating CUDA Backend...");
        set_backend(Backend::CUDA);
        println!("There are {} CUDA compute devices", device_count());
        test_backend();
    }

    if available.contains(&Backend::OPENCL) {
        println!("Evaluating OpenCL Backend...");
        set_backend(Backend::OPENCL);
        println!("There are {} OpenCL compute devices", device_count());
        test_backend();
    }

}
