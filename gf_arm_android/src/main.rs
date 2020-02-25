#[macro_use]
extern crate clap;

use std::fs::File;
use std::io::{Read,Write};

// [Macro clap::clap_app](https://docs.rs/clap/2.33.0/clap/macro.clap_app.html)

fn gpu_freq_stat(tail: &str) -> String {
    format!("/sys/class/devfreq/gpufreq/{}", tail)
}

fn ddr_freq_stat(tail: &str) -> String {
    format!("/sys/class/devfreq/ddrfreq/{}", tail)
}

fn cpu_freq_stat(id: usize, tail: &str) -> String {
    format!("/sys/devices/system/cpu/cpu{}/cpufreq/{}", id, tail)
}

struct AvailableFrequencies
{
    gpu: Vec<u64>,
    ddr: Vec<u64>,
    cpu_little: Vec<u64>,
    cpu_medium: Vec<u64>,
    cpu_big: Vec<u64>,
}

struct ActualFrequencies
{
    gpu: u64,
    ddr: u64,
    cpu_little: u64,
    cpu_medium: u64,
    cpu_big: u64,
}

impl ActualFrequencies {
    fn cons() -> ActualFrequencies {
        ActualFrequencies { gpu: 0, ddr: 0, cpu_little: 0, cpu_medium: 0, cpu_big: 0, }
    }

    fn init(gpu: u64, ddr: u64, cpu_l: u64, cpu_m: u64, cpu_b: u64) -> ActualFrequencies {
        ActualFrequencies { gpu: gpu, ddr: ddr, cpu_little: cpu_l, cpu_medium: cpu_m, cpu_big: cpu_b, }
    }
}

fn read_sys_record(sys_path: &str) -> String {
    let mut read_buf = String::new();
    if let Ok(mut file) = File::open(sys_path) {
        let sz = file.read_to_string(&mut read_buf).expect("cannot read sysfs");
        assert!(sz > 0);
    }
    read_buf.trim().to_string()
}

fn read_sys_as_u64(sys_path: &str) -> u64 {
    let str_buf = read_sys_record(sys_path);
    str_buf.parse().expect("cannot parse to u64")
}

fn read_gpu_frequency() -> u64 {
    read_sys_as_u64(&gpu_freq_stat("cur_freq"))
}

fn get_device_available_frequencies() -> AvailableFrequencies {
    fn get_sys_freqs(sys_path: &str) -> Vec<u64> {
        let mut freqs: Vec<u64> = Vec::new();
        if let Ok(mut file) = File::open(sys_path) {
            let mut read_buf = String::new();
            let sz = file.read_to_string(&mut read_buf).expect("cannot read sysfs");
            assert!(sz > 0);
            let vec: Vec<&str> = read_buf.trim().split(' ').collect();
            freqs = vec.into_iter().map(|x| x.parse().expect("cannot parse to u64")).collect();
        }
        freqs
    }
    AvailableFrequencies { gpu: get_sys_freqs(&gpu_freq_stat("available_frequencies")),
                           ddr: Vec::new(), cpu_little: Vec::new(), cpu_medium: Vec::new(), cpu_big: Vec::new(), }
}

fn main() {
    let matches = clap_app!(gf =>
        (version: "0.1")
        (author: "Alexander Samoilov <alexander.samoilov@gmail.com>")
        (about: "ARM Android cross-compile sample project")
        (@arg AVAILABLE: -a --available "display available frequencies")
        (@arg SHOW: -s --show "show current frequencies")
        (@arg SET_GPU: -g --gpu +takes_value "set GPU frequency")
        (@arg LOCK_FREQ: -l --lock ... +takes_value "lock GPU/DDR/CPU frequencies")
        (@arg VERBOSE: -v --verbose "be verbose")
    ).get_matches();

    if matches.is_present("VERBOSE") {
        println!("gf_freq command line args matches: {:?}", matches);
    }

    if matches.is_present("AVAILABLE") {
        let avail = get_device_available_frequencies();
        println!("GPU available frequencies: {:?}", avail.gpu);
    }

    if matches.is_present("SHOW") {
        println!("show device current frequencies: gpu: {}", read_gpu_frequency());
    }

    if let Ok(gpu_freq) = value_t!(matches, "SET_GPU", u64) {
        println!("Set GPU frequency to: {}", gpu_freq);
    }

    if let Ok(freq_vec) = values_t!(matches, "LOCK_FREQ", u64) {
        println!("freq_vec: {:?}", freq_vec);
        for v in freq_vec {
            println!("Set frequency to: {}", v);
        }
    }

}
