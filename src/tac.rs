#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VReg(pub u32);

#[derive(Debug, Clone)]
pub enum Value {
    Immediate(u64),
    VReg(VReg),
    DataLabel(usize),
}

#[derive(Debug, Clone)]
pub enum Operation {
    Const(u64),
    LoadAddr(usize),
    Syscall(Vec<Value>),
}

#[derive(Debug, Clone)]
pub struct Instruction {
    pub dest: Option<VReg>,
    pub op: Operation,
}

pub struct Program {
    pub instructions: Vec<Instruction>,
    pub data: Vec<u8>,
}

impl Program {
    pub fn hello_world() -> Self {
        let v0 = VReg(0);
        let v1 = VReg(1);
        let v2 = VReg(2);
        let v3 = VReg(3);

        Program {
            instructions: vec![
                Instruction {
                    dest: Some(v0),
                    op: Operation::Const(1),
                },
                Instruction {
                    dest: Some(v1),
                    op: Operation::LoadAddr(0),
                },
                Instruction {
                    dest: Some(v2),
                    op: Operation::Const(12),
                },
                Instruction {
                    dest: Some(v3),
                    op: Operation::Const(64),
                },
                Instruction {
                    dest: None,
                    op: Operation::Syscall(vec![
                        Value::VReg(v3),
                        Value::VReg(v0),
                        Value::VReg(v1),
                        Value::VReg(v2),
                    ]),
                },
                Instruction {
                    dest: Some(v0),
                    op: Operation::Const(0),
                },
                Instruction {
                    dest: Some(v3),
                    op: Operation::Const(93),
                },
                Instruction {
                    dest: None,
                    op: Operation::Syscall(vec![
                        Value::VReg(v3),
                        Value::VReg(v0),
                    ]),
                },
            ],
            data: b"Hello World\n".to_vec(),
        }
    }
}
