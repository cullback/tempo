use crate::ir::Register;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VReg(pub u32);

pub struct RegisterAllocator {
    allocation: HashMap<VReg, Register>,
}

impl RegisterAllocator {
    pub fn new() -> Self {
        Self {
            allocation: HashMap::new(),
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

    pub fn allocate(&mut self, vreg: VReg) -> Register {
        if let Some(&reg) = self.allocation.get(&vreg) {
            return reg;
        }

        let physical = match vreg.0 {
            0 => Register::X0,
            1 => Register::X1,
            2 => Register::X2,
            _ => Register::X8,
        };

        self.allocation.insert(vreg, physical);
        physical
    }

    pub fn get(&self, vreg: VReg) -> Register {
        *self.allocation.get(&vreg).unwrap_or_else(|| {
            panic!("Virtual register {:?} not allocated", vreg)
        })
    }
}
