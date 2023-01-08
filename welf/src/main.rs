use capstone::prelude::*;
use clap::Parser;
use goblin::elf::sym::{Sym, Symtab};
use goblin::{error, Object};
use std::collections::HashSet;
use std::error::Error;
use std::fs;
use std::path::Path;
use std::{fs::File, io::Read, path::PathBuf, str::FromStr};
use lief_cwal as lief;
use lief::{Binary, VerificationChecks, VerificationFlags};

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
    #[arg(required = false, short, long, num_args(1..))] // at least one section, such as .text
    disasm: Vec<String>,

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

#[derive(Debug)]
enum Arch {
    X86_64,
    AARCH64,
    UNKNOWN,
}

fn get_arch(header: &goblin::elf::Header) -> Arch {
    use goblin::elf::header::*;
    match header.e_machine {
        EM_X86_64  => Arch::X86_64,
        EM_AARCH64 => Arch::AARCH64,
        _          => Arch::UNKNOWN,
    }
}

fn disasm(bytes: &[u8], addr: u64, arch: &Arch) {
    let cs = match *arch {
        Arch::X86_64 => Capstone::new()
                            .x86()
                            .mode(arch::x86::ArchMode::Mode64)
                            .build()
                            .expect("failed to create capstone handle"),
        Arch::AARCH64 => Capstone::new()
                            .arm64()
                            .mode(arch::arm64::ArchMode::Arm)
                            .build()
                            .expect("failed to create capstone handle"),
        _             => panic!("not supported arch"),
    };
    match cs.disasm_all(bytes, addr) {
        Ok(insns) => {
            println!("disassembled {} instructions", insns.len());
            for i in insns.iter() {
                println!("{}", i);
            }
        }
        Err(err) => {
            println!("error {} while disassembling", err);
        }
    };
}

fn elf_summary(bytes: &Vec<u8>, args: &Args) {
    match goblin::elf::Elf::parse(&bytes) {
        Ok(binary) => {
            if args.verbose >= 4 {
                println!("elf itself: {:#x?}", &binary);
            }
            //let entry = binary.entry;
            //for ph in binary.program_headers {
            //    println!("ph: {:#x?}", &ph);
            //}
            let arch = get_arch(&binary.header);
            let sects_to_disasm: HashSet<String> = HashSet::from_iter(args.disasm.clone());
            if args.verbose >= 3 {
                println!("sects_to_disasm: {:#x?}", &sects_to_disasm);
            }
            for sh in binary.section_headers {
                let sect_name = binary
                    .shdr_strtab
                    .get_at(sh.sh_name)
                    .unwrap_or("")
                    .to_string();
                println!("section {} {:#x?}", &sect_name, &sh);
                if sects_to_disasm.contains(&sect_name) {
                    if args.verbose >= 3 {
                        println!("disassembling the section {}", &sect_name);
                    }
                    let offset = sh.sh_offset as usize;
                    disasm(&bytes[offset..offset + sh.sh_size as usize], sh.sh_addr, &arch);
                }
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
    if args.verbose >= 2 {
        println!("args: {:#x?}", &args);
    }
    let elves_path = Path::new(&args.elf);
    let buffer: Vec<u8> = fs::read(elves_path)?;
    elf_summary(&buffer, &args);
    // the same with lief
    let path = PathBuf::from_str(&args.elf).unwrap();
    let binary = Binary::new(path).unwrap();
    // println!("binary: {:#x?}", &binary); // `Binary` doesn't implement `Debug`
    Ok(())
}
