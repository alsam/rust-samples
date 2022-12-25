use goblin::elf::sym::{Sym, Symtab};

#[repr(C)]
#[repr(align(64))] // Align to cache lines
pub struct AlignedData<T: ?Sized>(T);

fn main() {
    println!("Hello, world!");
}
