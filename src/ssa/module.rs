use crate::ssa::basic_block::BasicBlock;

/// A module contains basic blocks and a data section.
#[derive(Clone, Default)]
pub struct Module {
    pub blocks: Vec<BasicBlock>,
    pub data: Vec<u8>,
}

impl Module {
    pub fn new() -> Self {
        Self::default()
    }
}
