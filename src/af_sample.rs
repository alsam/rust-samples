// Arrayfire sample

// Aftype -> DType [compilation errors with sample code](https://github.com/arrayfire/arrayfire-rust/issues/75)

extern crate arrayfire as af;

use af::*;

fn main() {
    let num_rows: u64 = 5;
    let num_cols: u64 = 3;
    let dims = Dim4::new(&[num_rows, num_cols, 1, 1]);
    //set_backend(Backend::AF_BACKEND_CPU);
    let available = get_available_backends();
    if available.contains(&Backend::CPU) {
        println!("Evaluating CPU Backend...");
        set_backend(Backend::CPU);
        println!("There are {} CPU compute devices", device_count());
    }

    //if available.contains(&Backend::CUDA) {
    //    println!("Evaluating CUDA Backend...");
    //    set_backend(Backend::CUDA);
    //    println!("There are {} CUDA compute devices", device_count());
    //    test_backend();
    //}

    //if available.contains(&Backend::OPENCL) {
    //    println!("Evaluating OpenCL Backend...");
    //    set_backend(Backend::OPENCL);
    //    println!("There are {} OpenCL compute devices", device_count());
    //    test_backend();
    //}

    println!("Create a 5-by-3 matrix of random floats on the GPU");
    let a = randu::<f32>(dims);
    print(&a);
}
