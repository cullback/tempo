use std::fs;
use std::process::Command;
use tempo::ast::{AstLowering, parse_program};
use tempo::backend::generate_elf_from_ir;
use tempo::ssa::lower;

fn compile_example(name: &str) -> Vec<u8> {
    let source = fs::read_to_string(format!("examples/{}.rb", name))
        .expect("Failed to read example file");

    let program = parse_program(&source).expect("Failed to parse");
    let lowering = AstLowering::new();
    let module = lowering.lower_program(&program);
    let ir_program = lower(&module);

    generate_elf_from_ir(&ir_program)
}

fn run_binary(binary: &[u8]) -> std::process::Output {
    use std::io::Write;
    use std::os::unix::fs::PermissionsExt;
    use std::sync::atomic::{AtomicU64, Ordering};

    static COUNTER: AtomicU64 = AtomicU64::new(0);
    let id = COUNTER.fetch_add(1, Ordering::Relaxed);
    let temp_path = format!("/tmp/tempo_test_{}_{}", std::process::id(), id);

    {
        let mut file =
            fs::File::create(&temp_path).expect("Failed to create temp file");
        file.write_all(binary).expect("Failed to write binary");
        file.sync_all().expect("Failed to sync file");
    } // Ensure file is closed before setting permissions

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
    let output = run_binary(&binary);

    assert_eq!(output.status.code(), Some(0));
    assert_eq!(String::from_utf8_lossy(&output.stdout), "Hello World\n");
    assert_eq!(binary.len(), 164);
}

#[test]
fn test_math() {
    let binary = compile_example("math");
    let output = run_binary(&binary);

    assert_eq!(output.status.code(), Some(220));
    assert_eq!(binary.len(), 164);
}

#[test]
fn test_conditional() {
    let binary = compile_example("conditional");
    let output = run_binary(&binary);

    assert_eq!(output.status.code(), Some(103));
    assert_eq!(binary.len(), 176);
}

#[test]
fn test_comparison() {
    let binary = compile_example("comparison");
    let output = run_binary(&binary);

    assert_eq!(output.status.code(), Some(20));
    assert_eq!(binary.len(), 176);
}
