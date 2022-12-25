use clap::Parser;
use goblin::elf::sym::{Sym, Symtab};

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

fn main() {
    let args = Args::parse();
    println!("I'm going to analyze elf {}", &args.elf);
}
