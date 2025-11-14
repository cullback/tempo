use std::fs;
use std::process::Command;
use tempo::ast::{AstLowering, parse_program};
use tempo::backend::{assemble_and_link_to_bytes, compile};
use tempo::ssa::lower;

fn compile_example(name: &str) -> Vec<u8> {
    let source = fs::read_to_string(format!("examples/{}.rb", name))
        .expect("Failed to read example file");

    let program = parse_program(&source).expect("Failed to parse");
    let lowering = AstLowering::new();
    let module = lowering.lower_program(&program);
    let ir_program = lower(&module);
    let asm = compile(&ir_program);

    assemble_and_link_to_bytes(&asm).expect("Failed to assemble and link")
}

fn run_binary(binary: &[u8]) -> std::process::Output {
    use std::io::Write;
    use std::os::unix::fs::PermissionsExt;
    use std::sync::atomic::{AtomicU64, Ordering};

    static COUNTER: AtomicU64 = AtomicU64::new(0);
    let id = COUNTER.fetch_add(1, Ordering::Relaxed);
    let temp_path = format!("/tmp/tempo_test_{}_{}", std::process::id(), id);

    let mut file =
        fs::File::create(&temp_path).expect("Failed to create temp file");
    file.write_all(binary).expect("Failed to write binary");
    drop(file); // Close the file before running

    let mut perms = fs::metadata(&temp_path).unwrap().permissions();
    perms.set_mode(0o755);
    fs::set_permissions(&temp_path, perms).unwrap();

    let output = Command::new(&temp_path)
        .output()
        .expect("Failed to run binary");

    fs::remove_file(&temp_path).ok();

    output
}

#[test]
fn test_hello() {
    let binary = compile_example("hello");
    let size = binary.len();
    let output = run_binary(&binary);

    assert_eq!(size, 376);
    assert_eq!(output.status.code(), Some(0));
    assert_eq!(String::from_utf8_lossy(&output.stdout), "Hello World\n");
}

#[test]
fn test_math() {
    let binary = compile_example("math");
    let size = binary.len();
    let output = run_binary(&binary);

    assert_eq!(size, 376);
    assert_eq!(output.status.code(), Some(220));
}

#[test]
fn test_conditional() {
    let binary = compile_example("conditional");
    let size = binary.len();
    let output = run_binary(&binary);

    assert_eq!(size, 392);
    assert_eq!(output.status.code(), Some(103));
}

#[test]
fn test_comparison() {
    let binary = compile_example("comparison");
    let size = binary.len();
    let output = run_binary(&binary);

    assert_eq!(size, 392);
    assert_eq!(output.status.code(), Some(20));
}
