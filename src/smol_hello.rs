use crate::aarch64_backend::AArch64Backend;
use crate::tac;
use crate::tac_lowering;

pub fn write_aarch64_hello() -> std::io::Result<()> {
    let tac_program = tac::Program::hello_world();
    let ir_program = tac_lowering::lower(&tac_program);
    let binary = AArch64Backend::compile(&ir_program);
    AArch64Backend::write_binary(&binary, "hello_aarch64")
}
