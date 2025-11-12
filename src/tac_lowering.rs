use crate::ir;
use crate::regalloc::RegisterAllocator;
use crate::tac;

pub fn lower(program: &tac::Program) -> ir::Program {
    let mut allocator = RegisterAllocator::new();
    let mut instructions = Vec::new();

    for instr in &program.instructions {
        match &instr.op {
            tac::Operation::Const(value) => {
                if let Some(dest) = instr.dest {
                    let physical = allocator.allocate(dest);
                    instructions.push(ir::Instruction::MovImm {
                        dest: physical,
                        value: *value,
                    });
                }
            }
            tac::Operation::LoadAddr(data_index) => {
                if let Some(dest) = instr.dest {
                    let physical = allocator.allocate(dest);
                    let code_size = program.instructions.len() * 4;
                    let data_offset = 0x78 + code_size;
                    let current_pc = 0x400078 + (instructions.len() * 4);
                    let target_addr = 0x400000 + data_offset + data_index;
                    let offset = (target_addr as i32) - (current_pc as i32);

                    instructions.push(ir::Instruction::AdrPcRel {
                        dest: physical,
                        offset,
                    });
                }
            }
            tac::Operation::Syscall(args) => {
                for (i, arg) in args.iter().enumerate() {
                    match arg {
                        tac::Value::VReg(vreg) => {
                            let src = allocator.get(*vreg);
                            let dest = allocator.allocate_for_syscall(*vreg, i);

                            if src != dest {
                                if let Some(value) = get_const_value(
                                    &program.instructions,
                                    *vreg,
                                ) {
                                    instructions.push(
                                        ir::Instruction::MovImm { dest, value },
                                    );
                                }
                            }
                        }
                        tac::Value::Immediate(value) => {
                            let dest = match i {
                                0 => ir::Register::X8,
                                1 => ir::Register::X0,
                                2 => ir::Register::X1,
                                3 => ir::Register::X2,
                                _ => panic!("Too many syscall arguments"),
                            };
                            instructions.push(ir::Instruction::MovImm {
                                dest,
                                value: *value,
                            });
                        }
                        tac::Value::DataLabel(_) => {
                            panic!(
                                "DataLabel not supported in syscall arguments"
                            )
                        }
                    }
                }
                instructions.push(ir::Instruction::Syscall);
            }
        }
    }

    ir::Program {
        instructions,
        data: program.data.clone(),
    }
}

fn get_const_value(
    instructions: &[tac::Instruction],
    vreg: tac::VReg,
) -> Option<u64> {
    for instr in instructions {
        if let Some(dest) = instr.dest {
            if dest == vreg {
                if let tac::Operation::Const(value) = instr.op {
                    return Some(value);
                }
            }
        }
    }
    None
}
