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
    #[must_use]
    pub fn build_module(self) -> Module {
        self.module
    }

    pub fn push_bb(&mut self) -> BlockId {
        let block = BasicBlock {
            params: Vec::new(),
            instructions: Vec::new(),
            terminator: Terminator::None,
        };
        let id = BlockId(self.module.blocks.len());
        self.module.blocks.push(block);
        id
    }

    pub fn switch_to_block(&mut self, id: BlockId) {
        self.current_block = Some(id);
    }

    pub fn set_terminator(&mut self, terminator: Terminator) {
        let id = self
            .current_block
            .expect("Malformed IR: Tried to set a terminator with no basic block selected.");
        self.module.blocks[id.0].terminator = terminator;
    }

    pub fn push_variable(&mut self) -> Value {
        let val = Value(self.val_counter);
        self.val_counter += 1;
        val
    }

    pub fn load_const(&mut self, var: Value, value: u64) {
        let id = self
            .current_block
            .expect("Malformed IR: Tried to load a const without a basic block selected.");
        self.module.blocks[id.0]
            .instructions
            .push(Instruction::Const(var, value));
    }

    pub fn load_data_addr(&mut self, var: Value, offset: usize) {
        let id = self
            .current_block
            .expect("Malformed IR: Tried to load data addr without a basic block selected.");
        self.module.blocks[id.0]
            .instructions
            .push(Instruction::LoadDataAddr(var, offset));
    }

    pub fn build_syscall(&mut self, args: Vec<Value>) {
        let id = self
            .current_block
            .expect("Malformed IR: Tried to build syscall without a basic block selected.");
        self.module.blocks[id.0]
            .instructions
            .push(Instruction::Syscall(None, args));
    }

    pub fn build_move(&mut self, from: Value, to: Value) {
        let id = self
            .current_block
            .expect("Malformed IR: Tried to build a move without a basic block selected.");
        self.module.blocks[id.0]
            .instructions
            .push(Instruction::Move(to, from));
    }

    pub fn build_binop(
        &mut self,
        to: Value,
        lhs: Value,
        rhs: Value,
        operation: BinaryOp,
    ) {
        let id = self.current_block.expect(
            "Malformed IR: Tried to build a binary operation without a basic block selected.",
        );
        self.module.blocks[id.0]
            .instructions
            .push(Instruction::BinOp(to, operation, lhs, rhs));
    }

    pub fn set_data(&mut self, data: Vec<u8>) {
        self.module.data = data;
    }

    pub fn build_branch(
        &mut self,
        condition: Value,
        then_block: BlockId,
        then_args: Vec<Value>,
        else_block: BlockId,
        else_args: Vec<Value>,
    ) {
        self.set_terminator(Terminator::Branch(
            condition, then_block, then_args, else_block, else_args,
        ));
    }

    pub fn build_jump(&mut self, target: BlockId, args: Vec<Value>) {
        self.set_terminator(Terminator::Jump(target, args));
    }

    pub fn add_block_param(&mut self, block: BlockId, param: Value) {
        self.module.blocks[block.0].params.push(param);
    }
}
