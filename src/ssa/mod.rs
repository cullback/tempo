pub mod basic_block;
pub mod instruction;
pub mod module;
pub mod module_builder;

pub use basic_block::{BasicBlock, BlockId, Terminator};
pub use instruction::{BinaryOp, Instruction, Value};
pub use module::Module;
pub use module_builder::ModuleBuilder;

const SYS_WRITE: u64 = 64;
const SYS_EXIT: u64 = 93;
const STDOUT: u64 = 1;

pub fn hello_world() -> Module {
    let mut builder = ModuleBuilder::new();

    builder.push_block(vec![]);

    let fd = builder.push_const(STDOUT);
    let buf = builder.push_load_data_addr(0);
    let len = builder.push_const(12);
    let write_syscall = builder.push_const(SYS_WRITE);

    builder.push_syscall(vec![write_syscall, fd, buf, len]);

    let exit_code = builder.push_const(0);
    let exit_syscall = builder.push_const(SYS_EXIT);

    builder.push_syscall(vec![exit_syscall, exit_code]);

    builder.terminate_return_void();

    builder.set_data(b"Hello World\n".to_vec());

    builder.build()
}
