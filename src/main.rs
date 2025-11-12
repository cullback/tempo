mod aarch64_backend;
mod ir;
mod regalloc;
mod smol_hello;
mod ssa;
mod ssa_lowering;

fn main() {
    smol_hello::write_aarch64_hello().unwrap();
}
