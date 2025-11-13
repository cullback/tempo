pub mod aarch64;
pub mod ir;
pub mod regalloc;

pub use aarch64::{assemble_and_link, compile, write_assembly};
