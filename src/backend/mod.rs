pub mod aarch64;
pub mod elf_bytes;
pub mod ir;
pub mod regalloc;

pub use aarch64::generate_elf_from_ir;
