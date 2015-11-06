
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
    full_rank: Vec<usize>, 
    reduced_rank: Vec<usize>, 
}

fn init_ranks(ki: &mut KernelIterator) {
    let num_models = ki.ref_mod.num_models;
    ki.full_rank.resize(num_models, 0);
    ki.reduced_rank.resize(num_models, 0);
    let mut total_kernels = 0;
    for i in 0..num_models {
        let nkernels = ki.ref_mod.models[i].num_kernels;
        ki.full_rank[i] = nkernels;
        total_kernels += nkernels;
    }
    let delta = total_kernels - ki.num_kernels;

    println!("total_kernels: {} num_kernels: {} delta: {}",
             total_kernels, ki.num_kernels, delta);

    for i in 0..num_models {
        let nkernels = ki.full_rank[i];
        let fraction = nkernels as f64 / total_kernels as f64;
        ki.reduced_rank[i] = nkernels - ((delta as f64 * fraction) as usize);
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

    println!("mod_array is constructed and properly initialized: {:?}", mod_array);

    let mut ki = KernelIterator {num_kernels : 17, state : (0, 0), ref_mod : &mod_array,
                                 full_rank: Vec::new(), reduced_rank: Vec::new()};
    init_ranks(&mut ki);

    println!("KernelIterator is constructed and properly initialized:m {:?}", ki);
}
