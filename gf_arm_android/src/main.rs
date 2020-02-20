#[macro_use]
extern crate clap;

// [Macro clap::clap_app](https://docs.rs/clap/2.33.0/clap/macro.clap_app.html)

fn main() {
    let matches = clap_app!(gf =>
        (version: "0.1")
        (author: "Alexander Samoilov <alexander.samoilov@gmail.com>")
        (about: "ARM Android cross-compile sample project")
        (@arg AVAILABLE: -a --available "display available frequencies")
        (@arg SET_GPU: -g --gpu +takes_value "set GPU frequency")
        (@arg LOCK_FREQ: -l --lock ... +takes_value "lock GPU/DDR/CPU frequencies as GPU/DDR/CPUlittle/CPUmiddle/CPUbig")
        (@arg VERBOSE: -v --verbose "be verbose")
    ).get_matches();

    if matches.is_present("VERBOSE") {
        println!("gf_freq command line args matches: {:?}", matches);
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
