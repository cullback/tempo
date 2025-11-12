use crate::aarch64_backend::AArch64Backend;
use crate::ssa_ir;
use crate::ssa_lowering;

pub fn write_aarch64_hello() -> std::io::Result<()> {
    let ssa_program = ssa_ir::Program::hello_world();
    let ir_program = ssa_lowering::lower(&ssa_program);
    let binary = AArch64Backend::compile(&ir_program);
    AArch64Backend::write_binary(&binary, "hello_aarch64")
}
