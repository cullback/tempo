use crate::backend::{
    ir,
    regalloc::{RegisterAllocator, VReg},
};
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

fn lower_blocks(
    blocks: &[crate::ssa::basic_block::BasicBlock],
    prefix: &str,
    syscall_positions: &HashMap<Value, usize>,
) -> Vec<ir::Instruction> {
    let mut instructions = Vec::new();
    let mut value_to_reg: HashMap<Value, ir::Register> = HashMap::new();
    let mut allocator = RegisterAllocator::new();

    let mut block_param_regs: HashMap<(usize, usize), ir::Register> =
        HashMap::new();

    for (block_id, block) in blocks.iter().enumerate() {
        for (param_idx, param) in block.params.iter().enumerate() {
            let vreg = VReg(param.0 as u32);
            let physical = if param_idx < 8 {
                match param_idx {
                    0 => ir::Register::X0,
                    1 => ir::Register::X1,
                    2 => ir::Register::X2,
                    3 => ir::Register::X3,
                    4 => ir::Register::X4,
                    5 => ir::Register::X5,
                    6 => ir::Register::X6,
                    7 => ir::Register::X7,
                    _ => unreachable!(),
                }
            } else {
                allocator.allocate(vreg)
            };
            block_param_regs.insert((block_id, param_idx), physical);
            value_to_reg.insert(*param, physical);
        }
    }

    for (block_id, block) in blocks.iter().enumerate() {
        instructions.push(ir::Instruction::Label {
            name: format!("{}{}", prefix, block_id),
        });

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
                Instruction::Call(dest, func_name, args) => {
                    for (i, arg) in args.iter().enumerate() {
                        if i < 8 {
                            let arg_reg = value_to_reg[arg];
                            let param_reg = match i {
                                0 => ir::Register::X0,
                                1 => ir::Register::X1,
                                2 => ir::Register::X2,
                                3 => ir::Register::X3,
                                4 => ir::Register::X4,
                                5 => ir::Register::X5,
                                6 => ir::Register::X6,
                                7 => ir::Register::X7,
                                _ => unreachable!(),
                            };
                            if arg_reg != param_reg {
                                instructions.push(ir::Instruction::Mov {
                                    dest: param_reg,
                                    src: arg_reg,
                                });
                            }
                        }
                    }

                    instructions.push(ir::Instruction::Call {
                        target: func_name.clone(),
                    });

                    let dest_reg = {
                        let vreg = VReg(dest.0 as u32);
                        allocator.allocate(vreg)
                    };
                    if dest_reg != ir::Register::X0 {
                        instructions.push(ir::Instruction::Mov {
                            dest: dest_reg,
                            src: ir::Register::X0,
                        });
                    }
                    value_to_reg.insert(*dest, dest_reg);
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

                    match op {
                        crate::ssa::BinaryOp::Add => {
                            instructions.push(ir::Instruction::Add {
                                dest: dest_reg,
                                src1: left_reg,
                                src2: right_reg,
                            });
                        }
                        crate::ssa::BinaryOp::Sub => {
                            instructions.push(ir::Instruction::Sub {
                                dest: dest_reg,
                                src1: left_reg,
                                src2: right_reg,
                            });
                        }
                        crate::ssa::BinaryOp::Mul => {
                            instructions.push(ir::Instruction::Mul {
                                dest: dest_reg,
                                src1: left_reg,
                                src2: right_reg,
                            });
                        }
                        crate::ssa::BinaryOp::Div => {
                            instructions.push(ir::Instruction::Div {
                                dest: dest_reg,
                                src1: left_reg,
                                src2: right_reg,
                            });
                        }
                        crate::ssa::BinaryOp::Eq => {
                            instructions.push(ir::Instruction::Cmp {
                                src1: left_reg,
                                src2: right_reg,
                            });
                            instructions.push(ir::Instruction::CSet {
                                dest: dest_reg,
                                condition: ir::Condition::Eq,
                            });
                        }
                        crate::ssa::BinaryOp::NotEq => {
                            instructions.push(ir::Instruction::Cmp {
                                src1: left_reg,
                                src2: right_reg,
                            });
                            instructions.push(ir::Instruction::CSet {
                                dest: dest_reg,
                                condition: ir::Condition::Ne,
                            });
                        }
                        crate::ssa::BinaryOp::Lt => {
                            instructions.push(ir::Instruction::Cmp {
                                src1: left_reg,
                                src2: right_reg,
                            });
                            instructions.push(ir::Instruction::CSet {
                                dest: dest_reg,
                                condition: ir::Condition::Lt,
                            });
                        }
                        crate::ssa::BinaryOp::Le => {
                            instructions.push(ir::Instruction::Cmp {
                                src1: left_reg,
                                src2: right_reg,
                            });
                            instructions.push(ir::Instruction::CSet {
                                dest: dest_reg,
                                condition: ir::Condition::Le,
                            });
                        }
                        crate::ssa::BinaryOp::Gt => {
                            instructions.push(ir::Instruction::Cmp {
                                src1: left_reg,
                                src2: right_reg,
                            });
                            instructions.push(ir::Instruction::CSet {
                                dest: dest_reg,
                                condition: ir::Condition::Gt,
                            });
                        }
                        crate::ssa::BinaryOp::Ge => {
                            instructions.push(ir::Instruction::Cmp {
                                src1: left_reg,
                                src2: right_reg,
                            });
                            instructions.push(ir::Instruction::CSet {
                                dest: dest_reg,
                                condition: ir::Condition::Ge,
                            });
                        }
                        _ => panic!("BinOp {:?} not yet implemented", op),
                    }

                    value_to_reg.insert(*dest, dest_reg);
                }
            }
        }

        use crate::ssa::Terminator;
        match &block.terminator {
            Terminator::None | Terminator::ReturnVoid => {}
            Terminator::Return(val) => {
                let ret_reg = value_to_reg[val];
                if ret_reg != ir::Register::X0 {
                    instructions.push(ir::Instruction::Mov {
                        dest: ir::Register::X0,
                        src: ret_reg,
                    });
                }
                instructions.push(ir::Instruction::Ret);
            }
            Terminator::Jump(target, args) => {
                for (arg_idx, arg_value) in args.iter().enumerate() {
                    let src_reg = value_to_reg[arg_value];
                    let dest_reg = block_param_regs[&(target.0, arg_idx)];
                    if src_reg != dest_reg {
                        instructions.push(ir::Instruction::Mov {
                            dest: dest_reg,
                            src: src_reg,
                        });
                    }
                }
                instructions.push(ir::Instruction::Jump {
                    target: format!("{}{}", prefix, target.0),
                });
            }
            Terminator::Branch(
                cond,
                then_block,
                then_args,
                else_block,
                else_args,
            ) => {
                let cond_reg = value_to_reg[cond];

                for (arg_idx, arg_value) in then_args.iter().enumerate() {
                    let src_reg = value_to_reg[arg_value];
                    let dest_reg = block_param_regs[&(then_block.0, arg_idx)];
                    if src_reg != dest_reg {
                        instructions.push(ir::Instruction::Mov {
                            dest: dest_reg,
                            src: src_reg,
                        });
                    }
                }

                instructions.push(ir::Instruction::Branch {
                    condition: cond_reg,
                    target: format!("{}{}", prefix, then_block.0),
                });

                for (arg_idx, arg_value) in else_args.iter().enumerate() {
                    let src_reg = value_to_reg[arg_value];
                    let dest_reg = block_param_regs[&(else_block.0, arg_idx)];
                    if src_reg != dest_reg {
                        instructions.push(ir::Instruction::Mov {
                            dest: dest_reg,
                            src: src_reg,
                        });
                    }
                }

                instructions.push(ir::Instruction::Jump {
                    target: format!("{}{}", prefix, else_block.0),
                });
            }
        }
    }

    instructions
}

pub fn lower(module: &Module) -> ir::Program {
    let mut instructions = Vec::new();
    let syscall_positions = analyze_syscall_usage(module);

    let main_instructions =
        lower_blocks(&module.blocks, ".Lblock", &syscall_positions);
    instructions.extend(main_instructions);

    for (func_name, func) in &module.functions {
        instructions.push(ir::Instruction::Label {
            name: func_name.clone(),
        });
        let func_syscall_positions = HashMap::new();
        let func_instructions = lower_blocks(
            &func.blocks,
            &format!(".L{}_block", func_name),
            &func_syscall_positions,
        );
        instructions.extend(func_instructions);
    }

    ir::Program {
        instructions,
        data: module.data.clone(),
    }
}
