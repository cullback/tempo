pub mod aarch64;
pub mod ir;
pub mod regalloc;

pub use aarch64::{assemble_and_link, assemble_and_link_to_bytes, compile};
