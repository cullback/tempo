use crate::ssa::basic_block::BasicBlock;
use std::collections::HashMap;
use std::fmt;

#[derive(Clone, Debug)]
pub struct Function {
    pub name: String,
    pub params: Vec<String>,
    pub blocks: Vec<BasicBlock>,
}

/// A module contains functions and a data section.
#[derive(Clone, Default, Debug)]
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

impl fmt::Display for Module {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if !self.blocks.is_empty() {
            writeln!(f, "Entry blocks:")?;
            for (i, block) in self.blocks.iter().enumerate() {
                writeln!(f, "  b{}:", i)?;
                for line in format!("{}", block).lines() {
                    writeln!(f, "    {}", line)?;
                }
            }
            writeln!(f)?;
        }

        if !self.functions.is_empty() {
            writeln!(f, "Functions:")?;
            for (name, func) in &self.functions {
                writeln!(f, "  {}:", name)?;
                for line in format!("{}", func).lines() {
                    writeln!(f, "    {}", line)?;
                }
            }
            writeln!(f)?;
        }

        if !self.data.is_empty() {
            write!(f, "Data: {:?}", self.data)?;
        }

        Ok(())
    }
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, block) in self.blocks.iter().enumerate() {
            writeln!(f, "b{}:", i)?;
            for line in format!("{}", block).lines() {
                writeln!(f, "  {}", line)?;
            }
        }
        Ok(())
    }
}
