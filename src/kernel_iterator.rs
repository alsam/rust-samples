
extern crate num;
use num::complex::Complex;

#[derive(Debug)]
struct ModelSingle {
    num_kernels: usize,
    kernels: Vec<Complex<f64>>,
}

#[derive(Debug)]
struct ModelArray {
    num_models: usize,
    models: Vec<ModelSingle>,
}

#[derive(Debug)]
struct KernelIterator<'a> {
    num_kernels: usize,
    state: (usize, usize),
    ref_mod: &'a ModelArray,
    full_rank: Vec<Vec<usize>>, 
    reduced_rank: Vec<Vec<usize>>, 
}

fn init_ranks(ki: &mut KernelIterator) {
    let num_models = ki.ref_mod.num_models;
    ki.full_rank.resize(num_models, Vec::new());
    ki.reduced_rank.resize(num_models, Vec::new());
    for z in 0..num_models {
    }
}

impl<'b> Iterator for KernelIterator<'b> {
    type Item = Vec<Complex<f64>>;
    fn next(&mut self) -> Option<Vec<Complex<f64>>> {
        None
    }
}

fn main() {
    println!("This is a kernel_iterator test");

    let kq1 = ModelSingle{num_kernels: 21, kernels: Vec::new()};
    let kq2 = ModelSingle{num_kernels: 75, kernels: Vec::new()};
    let kq3 = ModelSingle{num_kernels: 121, kernels: Vec::new()};

    let mut mod_array = ModelArray {num_models: 3, models: Vec::new()};
    mod_array.models.push(kq1);
    mod_array.models.push(kq2);
    mod_array.models.push(kq3);

    println!("mod_array constructed and properly initialized: {:?}", mod_array);

    let mut ki = KernelIterator {num_kernels : 17, state : (0, 0), ref_mod : &mod_array,
                                 full_rank: Vec::new(), reduced_rank: Vec::new()};
}
