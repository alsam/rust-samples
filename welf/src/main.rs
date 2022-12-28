use clap::Parser;
use goblin::elf::sym::{Sym, Symtab};
use goblin::{error, Object};
use std::error::Error;
use std::fs;
use std::path::Path;
use std::collections::HashSet;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the input elf
    #[arg(short, long)]
    elf: String,

    /// Print *all* syms
    #[arg(short, long)]
    all_syms: bool,

    /// Dissassemble given sections
    #[arg(short, long, num_args(1..))] // at least one section, such as .text
    disasm: Option<Vec<String>>,

    /// Reserved parameter for future use
    #[arg(short, long, default_value_t = 1)]
    count: u8,

    /// Verbosity level
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,
}

#[repr(C)]
#[repr(align(64))] // Align to cache lines
pub struct AlignedData<T: ?Sized>(T);

fn disasm(bytes: &Vec<u8>) {

}

fn elf_summary(bytes: &Vec<u8>, args: &Args) {
    match goblin::elf::Elf::parse(&bytes) {
        Ok(binary) => {
            //println!("elf itself: {:#x?}", &binary);
            //let entry = binary.entry;
            //for ph in binary.program_headers {
            //    println!("ph: {:#x?}", &ph);
            //}
            let sects_to_disasm: Vec<String> = args.disasm.as_ref().unwrap_or(&Vec::new()).clone();
            let hash_sects_to_disasm: HashSet<String> = HashSet::from_iter(sects_to_disasm);
            if args.verbose >= 3 { println!("hash_sects_to_disasm: {:#x?}", &hash_sects_to_disasm); }
            for sh in binary.section_headers {
                let sect_name = binary.shdr_strtab.get_at(sh.sh_name).unwrap_or("");
                println!("section {} {:#x?}", &sect_name, &sh);
            }
            if args.all_syms {
                let syms = binary.syms.to_vec();
                for sym in syms {
                    let sym_name = binary.strtab.get_at(sym.st_name).unwrap_or("");
                    println!("sym: {:#x?} with name {}", &sym, &sym_name);
                }
            }
        }
        Err(msg) => println!("fatal: {:?}", &msg),
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    if args.verbose >= 2 { println!("args: {:#x?}", &args); }
    let elves_path = Path::new(&args.elf);
    let buffer: Vec<u8> = fs::read(elves_path)?;
    elf_summary(&buffer, &args);
    Ok(())
}
