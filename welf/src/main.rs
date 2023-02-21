use capstone::prelude::*;
use clap::Parser;
use core::ops::Range;
use cpp_demangle::Symbol;
//use byteorder::{LittleEndian; ReadBytexExt}
use goblin::elf::sym::{Sym, Symtab};
use goblin::{error, Object};
//use lief::{Binary, VerificationChecks, VerificationFlags};
//use lief_cwal as lief;
use std::collections::HashSet;
use std::error::Error;
use std::fs;
use std::path::Path;
use std::{fs::File, io::Read, path::PathBuf, str::FromStr};

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

fn build_capstone_handle(
    header: &goblin::elf::Header,
) -> Result<capstone::Capstone, capstone::Error> {
    use goblin::elf::header::*;
    let capstone_new = Capstone::new();
    match header.e_machine {
        EM_X86_64 => capstone_new.x86().mode(arch::x86::ArchMode::Mode64).build(),
        EM_AARCH64 => capstone_new
            .arm64()
            .mode(arch::arm64::ArchMode::Arm)
            .build(),
        _ => unimplemented!(),
    }
}

fn disasm(bytes: &[u8], addr: u64, cs: &Capstone) {
    match cs.disasm_all(bytes, addr) {
        Ok(insns) => {
            println!("disassembled {} instructions", insns.len());
            for i in insns.iter() {
                println!("{}", i);
            }
        }
        Err(err) => {
            println!("error {err} while disassembling");
        }
    };
}

enum SectionRanges {
    Text(Range<u64>),
    Data(Range<u64>),
}

//pub type ElfImage<'a> = goblin::elf::Elf<'a>;

#[derive(Debug)]
struct ElfSummary {
    text: Range<u64>,
    data: Range<u64>,
    //elf_image: &goblin::elf::Elf,
}

impl ElfSummary {
    pub fn new(elf_image: &goblin::elf::Elf) -> ElfSummary {
        let sym_sh_name = |idx| elf_image.shdr_strtab.get_at(idx).unwrap_or_default();
        let (mut text_range, mut data_range): (Range<u64>, Range<u64>) = (0..0, 0..0);
        for sh in &elf_image.section_headers {
            let sect_name = sym_sh_name(sh.sh_name);
            let sec_beg = sh.sh_offset as u64;
            let sec_end = sec_beg + sh.sh_size as u64;
            let r: Range<u64> = sec_beg..sec_end;
            match sect_name {
                ".data" => {
                    data_range = r;
                    println!(
                        "data_range: {:#8x}..{:#8x}",
                        data_range.start, data_range.end
                    );
                }
                ".text" => {
                    text_range = r;
                    println!(
                        "text_range: {:#8x}..{:#8x}",
                        text_range.start, text_range.end
                    );
                }
                &_ => {} //todo!(),
            }
        }
        ElfSummary {
            text: text_range,
            data: data_range,
            //elf_image: elf_image,
        }
    }

    #[inline]
    pub fn in_text(&self, addr: &u64) -> bool {
        self.text.contains(&addr)
    }

    #[inline]
    pub fn in_data(&self, addr: &u64) -> bool {
        self.data.contains(&addr)
    }
}

fn elf_summary(bytes: &[u8], esum: &ElfSummary, args: &Args) {
    match goblin::elf::Elf::parse(&bytes) {
        Ok(binary) => {
            if args.verbose >= 4 {
                println!("elf itself: {:#x?}", &binary);
            }
            //let entry = binary.entry;
            //for ph in binary.program_headers {
            //    println!("ph: {:#x?}", &ph);
            //}
            match build_capstone_handle(&binary.header) {
                Ok(cs) => {
                    let vec_ref_sects: Vec<&str> = args.disasm.iter().map(|s| &**s).collect();
                    let sects_to_disasm: HashSet<&str> = HashSet::from_iter(vec_ref_sects);
                    if args.verbose >= 3 {
                        println!("sects_to_disasm: {:#x?}", &sects_to_disasm);
                    }
                    for sh in binary.section_headers {
                        let sect_name = binary.shdr_strtab.get_at(sh.sh_name).unwrap_or_default();
                        println!("section {} {:#x?}", &sect_name, &sh);
                        if sects_to_disasm.contains(&sect_name) {
                            if args.verbose >= 3 {
                                println!("disassembling the section {}", &sect_name);
                            }
                            let offset = sh.sh_offset as usize;
                            disasm(
                                &bytes[offset..offset + sh.sh_size as usize],
                                sh.sh_addr,
                                &cs,
                            );
                        }
                        const ADDRESS_SIZE: usize = 8;
                        match sect_name {
                            ".eh_frame_hdr" => {
                                let offset = sh.sh_offset as usize;
                                let bases = gimli::BaseAddresses::default().set_eh_frame_hdr(0);
                                let eh_frame_hdr = gimli::read::EhFrameHdr::new(
                                    &bytes[offset..],
                                    gimli::LittleEndian,
                                );
                                let parsed_eh_frame_hdr =
                                    eh_frame_hdr.parse(&bases, ADDRESS_SIZE as u8).unwrap();
                                println!(
                                    "eh frame pointer: {:#x?}, CFI table: {:#x?}",
                                    &parsed_eh_frame_hdr.eh_frame_ptr(),
                                    &parsed_eh_frame_hdr.table()
                                );
                            }

                            // ground truths and heuristics (H) about Vtables:
                            // H-1 Vtables have to lie in read-only sections.
                            // H-2 In a candidate vtable, only the beginning of the
                            //     function entries is referenced from the code.
                            // H-3 Offset-to-Top lies within a well-defined range and it
                            //     is no relocation entry.
                            // H-4 RTTI either points into a data .data section or is 0.
                            // H-5 A function entry points into a .text code section or is a
                            //     relocation entry.
                            // H-6 (relaxing) The first two function entries may be 0.
                            ".rodata" | ".rdata" | ".data.rel.ro" | ".data.rel.ro.local" => {
                                // vtable
                                let offset = sh.sh_offset as usize;
                                let scan_end = offset + sh.sh_size as usize;
                                for o in (offset..scan_end).step_by(ADDRESS_SIZE.into()) {
                                    let byte_slice =
                                        <[u8; ADDRESS_SIZE]>::try_from(&bytes[o..o + ADDRESS_SIZE])
                                            .unwrap();
                                    let addr = u64::from_le_bytes(byte_slice);
                                    println!(
                                        "addr: {:#8x} in_text: {} in_data: {}",
                                        addr,
                                        esum.in_text(&addr),
                                        esum.in_data(&addr)
                                    );
                                    println!("addr: {:?}", &byte_slice);
                                }
                            }
                            _ => { // just skip it
                            }
                        }
                    }
                }
                Err(err) => {
                    println!("error {err} while building capstone handle");
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
    let elf_image = goblin::elf::Elf::parse(&buffer).unwrap();
    let esummary = ElfSummary::new(&elf_image);
    println!("esummary: {:?}", &esummary);
    elf_summary(&buffer, &esummary, &args);
    // the same with lief
    //let path = PathBuf::from_str(&args.elf).unwrap();
    //let binary = Binary::new(path).unwrap();
    // println!("binary: {:#x?}", &binary); // `Binary` doesn't implement `Debug`
    Ok(())
}
