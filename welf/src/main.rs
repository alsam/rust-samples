use clap::Parser;
use goblin::{error, Object};
use goblin::elf::sym::{Sym, Symtab};
use std::path::Path;
use std::fs;

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

fn main() -> error::Result<()> {
   let args = Args::parse();
   println!("I'm going to analyze elf {}", &args.elf);
   let elves_path = Path::new(&args.elf);
   let buffer = fs::read(elves_path)?;
   match goblin::elf::Elf::parse(&buffer) {
      Ok(binary) => {
         println!("elf itself: {:?}",&binary);
         let entry = binary.entry;
         for ph in binary.program_headers {
            println!("ph: {:?}",&ph);
         }
         let syms = binary.syms.to_vec();
         for sym in syms {
            let sym_name = binary.strtab.get_at(sym.st_name).unwrap_or("");
            println!("sym: {:?} with name {}", &sym, &sym_name);
         }
      },
      Err(_) => ()
   }

   Ok(())
}
