fn write_elf_header(
    buf: &mut Vec<u8>,
    entry_point: u64,
    phoff: u64,
    phnum: u16,
) {
    // ELF Header (64 bytes)
    // e_ident
    buf.extend_from_slice(b"\x7fELF"); // Magic
    buf.push(2); // 64-bit
    buf.push(1); // Little endian
    buf.push(1); // ELF version
    buf.push(0); // System V ABI
    buf.extend_from_slice(&[0; 8]); // Padding

    // e_type: ET_EXEC (2)
    buf.extend_from_slice(&2u16.to_le_bytes());
    // e_machine: EM_AARCH64 (183)
    buf.extend_from_slice(&183u16.to_le_bytes());
    // e_version: 1
    buf.extend_from_slice(&1u32.to_le_bytes());
    // e_entry
    buf.extend_from_slice(&entry_point.to_le_bytes());
    // e_phoff (program header offset)
    buf.extend_from_slice(&phoff.to_le_bytes());
    // e_shoff: 0 (no section headers)
    buf.extend_from_slice(&0u64.to_le_bytes());
    // e_flags: 0
    buf.extend_from_slice(&0u32.to_le_bytes());
    // e_ehsize: 64
    buf.extend_from_slice(&64u16.to_le_bytes());
    // e_phentsize: 56 (program header entry size)
    buf.extend_from_slice(&56u16.to_le_bytes());
    // e_phnum
    buf.extend_from_slice(&phnum.to_le_bytes());
    // e_shentsize: 0
    buf.extend_from_slice(&0u16.to_le_bytes());
    // e_shnum: 0
    buf.extend_from_slice(&0u16.to_le_bytes());
    // e_shstrndx: 0
    buf.extend_from_slice(&0u16.to_le_bytes());
}

fn write_program_header(
    buf: &mut Vec<u8>,
    p_type: u32,
    p_flags: u32,
    p_offset: u64,
    p_vaddr: u64,
    p_filesz: u64,
    p_memsz: u64,
    p_align: u64,
) {
    // Program Header (56 bytes)
    buf.extend_from_slice(&p_type.to_le_bytes());
    buf.extend_from_slice(&p_flags.to_le_bytes());
    buf.extend_from_slice(&p_offset.to_le_bytes());
    buf.extend_from_slice(&p_vaddr.to_le_bytes());
    buf.extend_from_slice(&p_vaddr.to_le_bytes()); // p_paddr = p_vaddr
    buf.extend_from_slice(&p_filesz.to_le_bytes());
    buf.extend_from_slice(&p_memsz.to_le_bytes());
    buf.extend_from_slice(&p_align.to_le_bytes());
}

fn generate_hello_code() -> Vec<u8> {
    let mut code = Vec::new();

    // mov x0, #1
    code.extend_from_slice(&0xd2800020u32.to_le_bytes());

    // adr x1, msg (data is right after code, 32 bytes away)
    // PC when adr executes is 0x40007c, data will be at 0x400098
    // offset = 28 bytes
    let data_offset = 28i32;
    let immlo = (data_offset & 0x3) as u32;
    let immhi = ((data_offset >> 2) & 0x7FFFF) as u32;
    let adr_instr = 0x10000000 | (immlo << 29) | (immhi << 5) | 1; // x1
    code.extend_from_slice(&adr_instr.to_le_bytes());

    // mov x2, #13
    code.extend_from_slice(&0xd28001a2u32.to_le_bytes());

    // mov x8, #64
    code.extend_from_slice(&0xd2800808u32.to_le_bytes());

    // svc #0
    code.extend_from_slice(&0xd4000001u32.to_le_bytes());

    // mov x0, #0
    code.extend_from_slice(&0xd2800000u32.to_le_bytes());

    // mov x8, #93
    code.extend_from_slice(&0xd2800ba8u32.to_le_bytes());

    // svc #0
    code.extend_from_slice(&0xd4000001u32.to_le_bytes());

    code
}

pub fn generate_elf(code: &[u8], data: &[u8]) -> Vec<u8> {
    let mut elf = Vec::new();

    let code_offset = 0x78u64;
    let code_vaddr = 0x400078u64;
    let entry_point = code_vaddr;

    let total_size = code.len() as u64 + data.len() as u64;

    // Write ELF header
    write_elf_header(&mut elf, entry_point, 64, 1);

    // Write program header
    write_program_header(
        &mut elf,
        1, // PT_LOAD
        7, // PF_R | PF_W | PF_X
        code_offset,
        code_vaddr,
        total_size,
        total_size,
        4, // alignment
    );

    // Write code
    elf.extend_from_slice(code);

    // Write data (right after code)
    elf.extend_from_slice(data);

    elf
}

pub fn generate_hello_elf() -> Vec<u8> {
    let code = generate_hello_code();
    let data = b"Hello World\n\0";
    generate_elf(&code, data)
}
