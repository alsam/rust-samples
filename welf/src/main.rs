use clap::Parser;
use goblin::elf::sym::{Sym, Symtab};
use goblin::{error, Object};
use std::error::Error;
use std::fs;
use std::path::Path;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the input elf
    #[arg(short, long)]
    elf: String,

    /// Reserved parameter for future use
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

#[repr(C)]
#[repr(align(64))] // Align to cache lines
pub struct AlignedData<T: ?Sized>(T);

fn elf_summary(bytes: &Vec<u8>) {
    match goblin::elf::Elf::parse(&bytes) {
        Ok(binary) => {
            println!("elf itself: {:#x?}", &binary);
            let entry = binary.entry;
            for ph in binary.program_headers {
                println!("ph: {:#x?}", &ph);
            }
            let syms = binary.syms.to_vec();
            for sym in syms {
                let sym_name = binary.strtab.get_at(sym.st_name).unwrap_or("");
                println!("sym: {:#x?} with name {}", &sym, &sym_name);
            }
        }
        Err(msg) => println!("fatal: {:?}", &msg),
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    println!("I'm going to analyze elf {}", &args.elf);
    let elves_path = Path::new(&args.elf);
    let buffer: Vec<u8> = fs::read(elves_path)?;
    elf_summary(&buffer);
    Ok(())
}
