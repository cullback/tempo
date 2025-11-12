use crate::ssa::basic_block::{BasicBlock, BlockId, Terminator};
use crate::ssa::instruction::{BinaryOp, Instruction, Value};
use crate::ssa::module::Module;

/// [`ModuleBuilder`] represents the struct used for building a module
#[derive(Clone, Default)]
pub struct ModuleBuilder {
    module: Module,
    val_counter: usize,
    current_block: Option<BlockId>,
}

impl ModuleBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn fresh_value(&mut self) -> Value {
        let val = Value(self.val_counter);
        self.val_counter += 1;
        val
    }

    pub fn push_block(&mut self, params: Vec<Value>) -> BlockId {
        let block = BasicBlock {
            params,
            instructions: Vec::new(),
            terminator: Terminator::None,
        };
        let id = BlockId(self.module.blocks.len());
        self.module.blocks.push(block);
        self.current_block = Some(id);
        id
    }

    pub fn switch_to_block(&mut self, block: BlockId) {
        self.current_block = Some(block);
    }

    fn current_block_mut(&mut self) -> &mut BasicBlock {
        let block_id = self.current_block.expect("No current block");
        &mut self.module.blocks[block_id.0]
    }

    pub fn push_const(&mut self, value: u64) -> Value {
        let dest = self.fresh_value();
        let instr = Instruction::Const(dest, value);
        self.current_block_mut().instructions.push(instr);
        dest
    }

    pub fn push_load_data_addr(&mut self, offset: usize) -> Value {
        let dest = self.fresh_value();
        let instr = Instruction::LoadDataAddr(dest, offset);
        self.current_block_mut().instructions.push(instr);
        dest
    }

    pub fn push_syscall(&mut self, args: Vec<Value>) -> Option<Value> {
        let instr = Instruction::Syscall(None, args);
        self.current_block_mut().instructions.push(instr);
        None
    }

    pub fn push_binop(
        &mut self,
        op: BinaryOp,
        lhs: Value,
        rhs: Value,
    ) -> Value {
        let dest = self.fresh_value();
        let instr = Instruction::BinOp(dest, op, lhs, rhs);
        self.current_block_mut().instructions.push(instr);
        dest
    }

    pub fn push_move(&mut self, src: Value) -> Value {
        let dest = self.fresh_value();
        let instr = Instruction::Move(dest, src);
        self.current_block_mut().instructions.push(instr);
        dest
    }

    pub fn terminate_return_void(&mut self) {
        self.current_block_mut().terminator = Terminator::ReturnVoid;
    }

    pub fn terminate_return(&mut self, value: Value) {
        self.current_block_mut().terminator = Terminator::Return(value);
    }

    pub fn terminate_jump(&mut self, target: BlockId, args: Vec<Value>) {
        self.current_block_mut().terminator = Terminator::Jump(target, args);
    }

    pub fn terminate_branch(
        &mut self,
        cond: Value,
        then_block: BlockId,
        then_args: Vec<Value>,
        else_block: BlockId,
        else_args: Vec<Value>,
    ) {
        self.current_block_mut().terminator = Terminator::Branch(
            cond, then_block, then_args, else_block, else_args,
        );
    }

    pub fn set_data(&mut self, data: Vec<u8>) {
        self.module.data = data;
    }

    pub fn build(self) -> Module {
        self.module
    }
}
