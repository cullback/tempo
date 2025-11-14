use crate::ssa::basic_block::BasicBlock;
use std::collections::HashMap;

#[derive(Clone)]
pub struct Function {
    pub name: String,
    pub params: Vec<String>,
    pub blocks: Vec<BasicBlock>,
}

/// A module contains functions and a data section.
#[derive(Clone, Default)]
pub struct Module {
    pub blocks: Vec<BasicBlock>,
    pub functions: HashMap<String, Function>,
    pub data: Vec<u8>,
}

impl Module {
    pub fn new() -> Self {
        Self::default()
    }
}
