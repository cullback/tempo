mod aarch64_backend;
mod ir;
mod smol_hello;

fn main() {
    smol_hello::write_aarch64_hello().unwrap();
}
