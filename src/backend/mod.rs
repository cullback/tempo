pub mod aarch64;
pub mod ir;
pub mod regalloc;

pub use aarch64::{compile, write_binary};
