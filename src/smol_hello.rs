use std::fs::File;
use std::io::Write;

pub fn write_aarch64_hello() -> std::io::Result<()> {
    let mut binary = Vec::new();

    // ELF Header (64 bytes)
    binary.extend_from_slice(&[
        0x7f, b'E', b'L', b'F', // Magic number
        2,    // 64-bit
        1,    // Little endian
        1,    // ELF version
        0,    // System V ABI
        0, 0, 0, 0, 0, 0, 0, 0, // Padding
        2, 0, // Executable file
        0xb7, 0x00, // AArch64 machine type
        1, 0, 0, 0, // ELF version
        0x78, 0x00, 0x40, 0x00, 0x00, 0x00, 0x00,
        0x00, // Entry point: 0x400078
        0x40, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, // Program header offset: 0x40
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, // Section header offset: 0
        0x00, 0x00, 0x00, 0x00, // Flags
        0x40, 0x00, // ELF header size: 64 bytes
        0x38, 0x00, // Program header size: 56 bytes
        0x01, 0x00, // Number of program headers: 1
        0x00, 0x00, // Section header size: 0
        0x00, 0x00, // Number of section headers: 0
        0x00, 0x00, // Section header string table index: 0
    ]);

    // Program Header (56 bytes)
    binary.extend_from_slice(&[
        0x01, 0x00, 0x00, 0x00, // Type: PT_LOAD
        0x05, 0x00, 0x00, 0x00, // Flags: PF_R | PF_X
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // Offset: 0
        0x00, 0x00, 0x40, 0x00, 0x00, 0x00, 0x00,
        0x00, // Virtual address: 0x400000
        0x00, 0x00, 0x40, 0x00, 0x00, 0x00, 0x00,
        0x00, // Physical address: 0x400000
        0x9c, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, // File size: 156 bytes
        0x9c, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, // Memory size: 156 bytes
        0x00, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, // Alignment: 0x1000
    ]);

    // Code section (starts at offset 0x78, virtual address 0x400078)
    // AArch64 assembly for "Hello World\n"
    binary.extend_from_slice(&[
        // mov x0, #1 (stdout)
        0x20, 0x00, 0x80, 0xd2,
        // adr x1, #28 (PC + 28 = string at offset 0x94)
        0xe1, 0x00, 0x00, 0x10, // mov x2, #12 (length)
        0x82, 0x01, 0x80, 0xd2, // mov x8, #64 (sys_write)
        0x08, 0x08, 0x80, 0xd2, // svc #0
        0x01, 0x00, 0x00, 0xd4, // mov x0, #0 (exit code)
        0x00, 0x00, 0x80, 0xd2, // mov x8, #93 (sys_exit)
        0xa8, 0x0b, 0x80, 0xd2, // svc #0
        0x01, 0x00, 0x00, 0xd4,
    ]);

    // "Hello World\n" string
    binary.extend_from_slice(b"Hello World\n");

    let mut file = File::create("hello_aarch64")?;
    file.write_all(&binary)?;

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = file.metadata()?.permissions();
        perms.set_mode(0o755);
        std::fs::set_permissions("hello_aarch64", perms)?;
    }

    println!("Created hello_aarch64 ({} bytes)", binary.len());
    Ok(())
}
