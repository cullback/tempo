use crate::aarch64_backend::AArch64Backend;
use crate::ssa::{ModuleBuilder, STDOUT, SYS_EXIT, SYS_WRITE, Terminator};
use crate::ssa_lowering;

pub fn write_aarch64_hello() -> std::io::Result<()> {
    let mut builder = ModuleBuilder::default();

    let block = builder.push_bb();
    builder.switch_to_block(block);

    let fd = builder.push_variable();
    builder.load_const(fd, STDOUT);

    let buf = builder.push_variable();
    builder.load_data_addr(buf, 0);

    let len = builder.push_variable();
    builder.load_const(len, 12);

    let write_syscall = builder.push_variable();
    builder.load_const(write_syscall, SYS_WRITE);

    builder.build_syscall(vec![write_syscall, fd, buf, len]);

    let exit_code = builder.push_variable();
    builder.load_const(exit_code, 0);

    let exit_syscall = builder.push_variable();
    builder.load_const(exit_syscall, SYS_EXIT);

    builder.build_syscall(vec![exit_syscall, exit_code]);

    builder.set_terminator(Terminator::ReturnVoid);

    builder.set_data(b"Hello World\n".to_vec());

    let module = builder.build_module();
    let ir_program = ssa_lowering::lower(&module);
    let binary = AArch64Backend::compile(&ir_program);
    AArch64Backend::write_binary(&binary, "hello_aarch64")
}
