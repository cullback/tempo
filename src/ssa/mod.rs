pub mod basic_block;
pub mod instruction;
pub mod module;
pub mod module_builder;

pub use basic_block::Terminator;
pub use instruction::{BinaryOp, Instruction, Value};
pub use module::Module;
pub use module_builder::ModuleBuilder;

pub const SYS_WRITE: u64 = 64;
pub const SYS_EXIT: u64 = 93;
pub const STDOUT: u64 = 1;
