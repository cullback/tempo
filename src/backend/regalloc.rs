use crate::backend::ir::Register;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VReg(pub u32);

pub struct RegisterAllocator {
    allocation: HashMap<VReg, Register>,
    reserved_registers: HashSet<Register>,
    next_register: usize,
}

impl RegisterAllocator {
    pub fn new() -> Self {
        Self {
            allocation: HashMap::new(),
            reserved_registers: HashSet::new(),
            next_register: 0,
        }
    }

    pub fn allocate_for_syscall(
        &mut self,
        vreg: VReg,
        syscall_position: usize,
    ) -> Register {
        let physical = match syscall_position {
            0 => Register::X8,
            1 => Register::X0,
            2 => Register::X1,
            3 => Register::X2,
            _ => panic!("Too many syscall arguments"),
        };

        self.allocation.insert(vreg, physical);
        physical
    }

    pub fn reserve(&mut self, vreg: VReg, physical: Register) {
        self.allocation.insert(vreg, physical);
        self.reserved_registers.insert(physical);
    }

    pub fn allocate(&mut self, vreg: VReg) -> Register {
        if let Some(&reg) = self.allocation.get(&vreg) {
            return reg;
        }

        let available_regs = [
            Register::X0,
            Register::X1,
            Register::X2,
            Register::X3,
            Register::X4,
            Register::X5,
            Register::X6,
            Register::X7,
        ];

        let start = self.next_register;
        loop {
            let physical =
                available_regs[self.next_register % available_regs.len()];
            self.next_register += 1;

            if !self.reserved_registers.contains(&physical) {
                self.allocation.insert(vreg, physical);
                return physical;
            }

            if self.next_register - start >= available_regs.len() {
                panic!(
                    "Register allocator: all {} registers are reserved! Virtual register {:?} cannot be allocated.",
                    available_regs.len(),
                    vreg
                );
            }
        }
    }

    pub fn get(&self, vreg: VReg) -> Register {
        *self.allocation.get(&vreg).unwrap_or_else(|| {
            panic!("Virtual register {:?} not allocated", vreg)
        })
    }
}
