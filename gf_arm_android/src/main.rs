// The MIT License (MIT)
//
// Copyright (c) 2020 Alexander Samoilov
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

#[macro_use]
extern crate clap;

use std::fs::File;
use std::io::{Read,Write};
use core::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;

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
    fn zero() -> ActualFrequencies {
        ActualFrequencies { gpu: 0, ddr: 0, cpu_little: 0, cpu_medium: 0, cpu_big: 0, }
    }

    fn new(gpu: u64, ddr: u64, cpu_l: u64, cpu_m: u64, cpu_b: u64) -> ActualFrequencies {
        ActualFrequencies { gpu: gpu, ddr: ddr, cpu_little: cpu_l, cpu_medium: cpu_m, cpu_big: cpu_b, }
    }
}

impl std::cmp::PartialEq for ActualFrequencies {

    #[inline]
    fn eq(&self, other: &ActualFrequencies) -> bool {
        self.gpu == other.gpu &&
        self.ddr == other.ddr &&
        self.cpu_little == other.cpu_little &&
        self.cpu_medium == other.cpu_medium &&
        self.cpu_big == other.cpu_big
    }

    #[inline]
    fn ne(&self, other: &ActualFrequencies) -> bool {
        !self.eq(other)
    }
}

struct FrequencyLocker {
    freqs: ActualFrequencies,
    runnable: Arc<AtomicBool>,
}

impl FrequencyLocker {
    fn new(freq_vec: &Vec<u64>) -> FrequencyLocker {
        FrequencyLocker {
            freqs: ActualFrequencies::new(freq_vec[0], freq_vec[1], freq_vec[2], freq_vec[3], freq_vec[4]),
            runnable: Arc::new(AtomicBool::new(true)),
        }
    }

    fn do_poll(&self) {
        while self.runnable.load(Ordering::SeqCst) {
            std::thread::sleep(std::time::Duration::from_millis(50));
            let cur_freqs = read_current_frequencies();
            if cur_freqs != self.freqs {
                lock_all_frequencies(&self.freqs)
            }
        }
    }
}

#[inline]
fn read_sys_str(sys_path: &str) -> String {
    let mut read_buf = String::new();
    if let Ok(mut file) = File::open(sys_path) {
        let sz = file.read_to_string(&mut read_buf).expect("cannot read from sysfs");
        assert!(sz > 0);
    }
    read_buf.trim().to_string()
}

#[inline]
fn read_sys_as_u64(sys_path: &str) -> u64 {
    let str_buf = read_sys_str(sys_path);
    str_buf.parse().expect("cannot parse to u64")
}

#[inline]
fn write_sys_str(sys_path: &str, val: &str) {
    if let Ok(mut file) = File::create(sys_path) {
        let sz = file.write(&val.as_bytes()).expect("cannot write to sysfs");
        assert!(sz > 0);
    }
}

#[inline]
fn write_sys_u64(sys_path: &str, val: u64) {
    write_sys_str(sys_path, &format!("{}", val))
}

#[inline]
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
                           ddr: Vec::new(),
                           cpu_little: Vec::new(),
                           cpu_medium: Vec::new(),
                           cpu_big: Vec::new(), }
}

#[inline]
fn read_current_frequencies() -> ActualFrequencies {
    ActualFrequencies {
        gpu: read_gpu_frequency(),
        ddr: 0,
        cpu_little: 0,
        cpu_medium: 0,
        cpu_big: 0,
    }
}

#[inline]
fn lock_sys_freq(freq: u64, cur_freq: u64, sys_min_freq: &str, sys_max_freq: &str)
{
    if freq < cur_freq {
        write_sys_u64(sys_min_freq, freq);
        write_sys_u64(sys_max_freq, freq)
    } else if freq > cur_freq {
        write_sys_u64(sys_max_freq, freq);
        write_sys_u64(sys_min_freq, freq)
    }
}

#[inline]
fn lock_gpu_frequency(freq: u64)
{
    lock_sys_freq(freq, read_gpu_frequency(), &gpu_freq_stat("min_freq"), &gpu_freq_stat("max_freq"))
}

#[inline]
fn lock_all_frequencies(freqs: &ActualFrequencies)
{
    lock_gpu_frequency(freqs.gpu);
}

fn main() {
    let matches = clap_app!(gf =>
        (version: "0.2")
        (author: "Alexander Samoilov <alexander.samoilov@gmail.com>")
        (about: "ARM Android cross-compile sample project")
        (@arg AVAILABLE: -a --available "display available frequencies")
        (@arg SHOW: -s --show "show current frequencies")
        (@arg SET_GPU: -g --gpu +takes_value "set GPU frequency")
        (@arg LOCK_FREQ: -l --lock ... +takes_value "lock GPU/DDR/CPU frequencies")
        (@arg VERBOSE: -v --verbose "be verbose")
    ).get_matches();

    if matches.is_present("VERBOSE") {
        println!("gf_freq command line args matches: {:?}", matches)
    }

    if matches.is_present("AVAILABLE") {
        let avail = get_device_available_frequencies();
        println!("GPU available frequencies: {:?}", avail.gpu)
    }

    if matches.is_present("SHOW") {
        println!("show device current frequencies: gpu: {}", read_gpu_frequency())
    }

    if let Ok(gpu_freq) = value_t!(matches, "SET_GPU", u64) {
        println!("Set GPU frequency to: {}", gpu_freq);
        lock_gpu_frequency(gpu_freq)
    }

    if let Ok(freq_vec) = values_t!(matches, "LOCK_FREQ", u64) {
        println!("freq_vec: {:?}", freq_vec);
        for v in freq_vec.clone() {
            println!("Set frequency to: {}", v)
        }

        let freq_locker = FrequencyLocker::new(&freq_vec);
        let poller = thread::spawn(move || {
            freq_locker.do_poll()
        });
        poller.join().expect("frequency locker thread has panicked");
    }

}
