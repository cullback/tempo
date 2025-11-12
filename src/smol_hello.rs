use crate::aarch64_backend::AArch64Backend;
use crate::ir::Program;

pub fn write_aarch64_hello() -> std::io::Result<()> {
    let program = Program::hello_world();
    AArch64Backend::compile(&program, "hello_aarch64")
}
