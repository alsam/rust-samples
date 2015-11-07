
extern crate num;
use num::complex::Complex;

#[derive(Debug)]
struct ModelSingle {
    num_kernels: usize,
    kernels: Vec<Vec<Complex<f64>>>,
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

    fn valid_state(&self) -> bool {
        let (model_index, kernel_index) = self.state;
        model_index  < self.ref_mod.num_models &&
        kernel_index < self.reduced_rank[model_index]
    }

    fn advance_state(&mut self) -> bool {
        let (model_index, kernel_index) = self.state;
        if kernel_index+1 < self.reduced_rank[model_index] {
            self.state = (model_index, kernel_index + 1);
            true
        } else if model_index < self.ref_mod.num_models {
            self.state = (model_index + 1, 0);
            true
        } else {
            self.state = (model_index + 1, kernel_index);
            false
        }
    }
}

impl<'c> Iterator for KernelIterator<'c> {
    type Item = Vec<Complex<f64>>;

    fn next(&mut self) -> Option<Vec<Complex<f64>>> {
        let ret = if self.valid_state() {
            let (model_index, kernel_index) = self.state;
            let m = self.ref_mod;
            let ref model = m.models[model_index];
            println!("model_index: {} kernel_index: {}", model_index, kernel_index);
            Some(model.kernels[kernel_index].clone())
        } else {
            None
        };
        self.advance_state();
        ret
    }
}

fn main() {
    println!("This is a kernel_iterator test");

    let kq1 = ModelSingle{num_kernels: 21,  kernels: vec![Vec::new(); 21]};
    let kq2 = ModelSingle{num_kernels: 75,  kernels: vec![Vec::new(); 75]};
    let kq3 = ModelSingle{num_kernels: 121, kernels: vec![Vec::new(); 121]};

    let mut mod_array = ModelArray {num_models: 3, models: Vec::new()};
    mod_array.models.push(kq1);
    mod_array.models.push(kq2);
    mod_array.models.push(kq3);

    println!("mod_array is constructed and properly initialized: {:?}", mod_array);

    let ki = KernelIterator::new(17, &mod_array);

    println!("KernelIterator is constructed and properly initialized: {:?}", ki);

    for _ in ki {
    }
}

