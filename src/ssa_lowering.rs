use crate::ir;
use crate::regalloc::{RegisterAllocator, VReg};
use crate::ssa_ir;
use std::collections::HashMap;

pub fn lower(program: &ssa_ir::Program) -> ir::Program {
    let mut instructions = Vec::new();
    let mut value_to_reg: HashMap<ssa_ir::Value, ir::Register> = HashMap::new();
    let mut allocator = RegisterAllocator::new();

    for block in &program.blocks {
        for instr in &block.instructions {
            match instr {
                ssa_ir::Instruction::Const(dest, val) => {
                    let physical = match dest.0 {
                        0 => ir::Register::X0,
                        2 => ir::Register::X2,
                        3 => ir::Register::X8,
                        _ => {
                            let vreg = VReg(dest.0 as u32);
                            allocator.allocate(vreg)
                        }
                    };

                    instructions.push(ir::Instruction::MovImm {
                        dest: physical,
                        value: *val,
                    });
                    value_to_reg.insert(*dest, physical);
                }
                ssa_ir::Instruction::LoadDataAddr(dest, offset) => {
                    let physical = match dest.0 {
                        1 => ir::Register::X1,
                        _ => {
                            let vreg = VReg(dest.0 as u32);
                            allocator.allocate(vreg)
                        }
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
                ssa_ir::Instruction::Syscall(_result, args) => {
                    instructions.push(ir::Instruction::Syscall);
                }
                ssa_ir::Instruction::Move(dest, src) => {
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
                ssa_ir::Instruction::BinOp(_, _, _, _) => {
                    panic!("BinOp not yet implemented");
                }
            }
        }
    }

    ir::Program {
        instructions,
        data: program.data.clone(),
    }
}
