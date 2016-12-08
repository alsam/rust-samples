// Arrayfire sample

// Aftype -> DType [compilation errors with sample code](https://github.com/arrayfire/arrayfire-rust/issues/75)

extern crate arrayfire as af;

use af::*;

// low dimension linear algebra type aliases
pub fn new_vec3<T: af::HasAfEnum>(values: &[T]) -> Array
{
    Array::new(&values, Dim4::new(&[3, 1, 1, 1]))
}

pub fn new_mat3x3<T: af::HasAfEnum>(values: &[T]) -> Array
{
    Array::new(&values, Dim4::new(&[3, 3, 1, 1]))
}

fn test_backend()
{
    let a: [f32; 3]  = [1.0, 2.0, 3.0];
    let va = new_vec3(&a);
    println!("a : {:?}", &a);
    print(&va);
    let b: [f32; 9]  = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0];
    let mb = new_mat3x3(&b);
    let mc = new_mat3x3(&b);
    println!("a : {:?}", &b);
    print(&mb);
    let dc = mb * mc;
    print(&dc);

    let mut dim = 0;

    dim = 7 * 3;

    let vb = Array::new(&b, Dim4::new(&[dim, 1, 1, 1]));
    print(&vb);
}

fn main() {
    let num_rows: u64 = 5;
    let num_cols: u64 = 3;
    let dims = Dim4::new(&[num_rows, num_cols, 1, 1]);
    let available = get_available_backends();
    if available.contains(&Backend::CPU) {
        println!("Evaluating CPU Backend...");
        set_backend(Backend::CPU);
        println!("There are {} CPU compute devices", device_count());
    }

    if available.contains(&Backend::CUDA) {
        println!("Evaluating CUDA Backend...");
        set_backend(Backend::CUDA);
        println!("There are {} CUDA compute devices", device_count());
        set_device(0);
        info();

        test_backend();
    }

    if available.contains(&Backend::OPENCL) {
        println!("Evaluating OpenCL Backend...");
        set_backend(Backend::OPENCL);
        println!("There are {} OpenCL compute devices", device_count());

        test_backend();
    }

    println!("Create a 5-by-3 matrix of random floats on the GPU");
    let a = randu::<f32>(dims);
    print(&a);
}
