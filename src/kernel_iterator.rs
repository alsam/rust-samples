
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

impl<'b> KernelIterator<'b> {
    fn new(num_kernels: usize, ref_mod: &'b ModelArray) -> KernelIterator {
        let num_models = ref_mod.num_models;
        let mut full_rank     = vec![0; num_models];
        let mut reduced_rank  = vec![0; num_models];
        let mut total_kernels = 0;
        for i in 0..num_models {
            let nkernels = ref_mod.models[i].num_kernels;
            full_rank[i] = nkernels;
            total_kernels += nkernels;
        }
        let delta = total_kernels - num_kernels;

        println!("total_kernels: {} num_kernels: {} delta: {}",
                 total_kernels, num_kernels, delta);

        for i in 0..num_models {
            let nkernels = full_rank[i];
            let fraction = nkernels as f64 / total_kernels as f64;
            reduced_rank[i] = nkernels - ((delta as f64 * fraction) as usize);
        }

        KernelIterator { num_kernels: num_kernels, state : (0, 0), ref_mod: ref_mod,
                         full_rank: full_rank, reduced_rank: reduced_rank}
    }
}

impl<'c> Iterator for KernelIterator<'c> {
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

    let mut ki = KernelIterator::new(17, &mod_array);

    println!("KernelIterator is constructed and properly initialized:m {:?}", ki);
}
