use crate::ir;
use crate::regalloc::{RegisterAllocator, VReg};
use crate::ssa::{Instruction, Module, Value};
use std::collections::HashMap;

fn analyze_syscall_usage(module: &Module) -> HashMap<Value, usize> {
    let mut syscall_positions = HashMap::new();

    for block in &module.blocks {
        for instr in &block.instructions {
            if let Instruction::Syscall(_, args) = instr {
                for (i, arg) in args.iter().enumerate() {
                    syscall_positions.insert(*arg, i);
                }
            }
        }
    }

    syscall_positions
}

pub fn lower(module: &Module) -> ir::Program {
    let mut instructions = Vec::new();
    let mut value_to_reg: HashMap<Value, ir::Register> = HashMap::new();
    let mut allocator = RegisterAllocator::new();
    let syscall_positions = analyze_syscall_usage(module);

    for block in &module.blocks {
        for instr in &block.instructions {
            match instr {
                Instruction::Const(dest, val) => {
                    let physical =
                        if let Some(&pos) = syscall_positions.get(dest) {
                            match pos {
                                0 => ir::Register::X8,
                                1 => ir::Register::X0,
                                2 => ir::Register::X1,
                                3 => ir::Register::X2,
                                _ => {
                                    let vreg = VReg(dest.0 as u32);
                                    allocator.allocate(vreg)
                                }
                            }
                        } else {
                            let vreg = VReg(dest.0 as u32);
                            allocator.allocate(vreg)
                        };

                    instructions.push(ir::Instruction::MovImm {
                        dest: physical,
                        value: *val,
                    });
                    value_to_reg.insert(*dest, physical);
                }
                Instruction::LoadDataAddr(dest, offset) => {
                    let physical =
                        if let Some(&pos) = syscall_positions.get(dest) {
                            match pos {
                                0 => ir::Register::X8,
                                1 => ir::Register::X0,
                                2 => ir::Register::X1,
                                3 => ir::Register::X2,
                                _ => {
                                    let vreg = VReg(dest.0 as u32);
                                    allocator.allocate(vreg)
                                }
                            }
                        } else {
                            let vreg = VReg(dest.0 as u32);
                            allocator.allocate(vreg)
                        };

                    let current_instr_count = instructions.len();
                    let estimated_remaining = 7;
                    let total_instrs =
                        current_instr_count + estimated_remaining;
                    let data_offset_in_file = 0x78 + (total_instrs * 4);
                    let current_pc = 0x400078 + (current_instr_count * 4);
                    let target_addr = 0x400000 + data_offset_in_file + offset;
                    let pc_offset = (target_addr as i32) - (current_pc as i32);

                    instructions.push(ir::Instruction::AdrPcRel {
                        dest: physical,
                        offset: pc_offset,
                    });
                    value_to_reg.insert(*dest, physical);
                }
                Instruction::Syscall(_result, _args) => {
                    instructions.push(ir::Instruction::Syscall);
                }
                Instruction::Move(dest, src) => {
                    let src_reg = value_to_reg[src];
                    let dest_reg =
                        value_to_reg.get(dest).copied().unwrap_or_else(|| {
                            let vreg = VReg(dest.0 as u32);
                            allocator.allocate(vreg)
                        });

                    if src_reg != dest_reg {
                        panic!(
                            "Move instruction not yet implemented for different registers"
                        );
                    }
                }
                Instruction::BinOp(dest, op, left, right) => {
                    let left_reg = value_to_reg[left];
                    let right_reg = value_to_reg[right];
                    let dest_reg = {
                        let vreg = VReg(dest.0 as u32);
                        allocator.allocate(vreg)
                    };

                    let ir_instr = match op {
                        crate::ssa::BinaryOp::Add => ir::Instruction::Add {
                            dest: dest_reg,
                            src1: left_reg,
                            src2: right_reg,
                        },
                        crate::ssa::BinaryOp::Sub => ir::Instruction::Sub {
                            dest: dest_reg,
                            src1: left_reg,
                            src2: right_reg,
                        },
                        crate::ssa::BinaryOp::Mul => ir::Instruction::Mul {
                            dest: dest_reg,
                            src1: left_reg,
                            src2: right_reg,
                        },
                        crate::ssa::BinaryOp::Div => ir::Instruction::Div {
                            dest: dest_reg,
                            src1: left_reg,
                            src2: right_reg,
                        },
                        _ => panic!("BinOp {:?} not yet implemented", op),
                    };

                    instructions.push(ir_instr);
                    value_to_reg.insert(*dest, dest_reg);
                }
            }
        }
    }

    ir::Program {
        instructions,
        data: module.data.clone(),
    }
}
