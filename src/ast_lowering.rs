use crate::ast::{
    Assignment, Block, Expression, FunctionCall, Identifier, Number, Program,
};
use crate::ssa::{
    Module, ModuleBuilder, SYS_EXIT, SYS_WRITE, Terminator, Value,
};
use std::collections::HashMap;

pub struct AstLowering<'a> {
    builder: ModuleBuilder,
    bindings: HashMap<String, Value>,
    data: Vec<u8>,
    _phantom: std::marker::PhantomData<&'a ()>,
}

impl<'a> AstLowering<'a> {
    pub fn new() -> Self {
        Self {
            builder: ModuleBuilder::default(),
            bindings: HashMap::new(),
            data: Vec::new(),
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn lower_program(mut self, program: &Program<'a>) -> Module {
        let block = self.builder.push_bb();
        self.builder.switch_to_block(block);

        for assignment in &program.assignments {
            self.lower_assignment(assignment);
        }

        self.builder.set_terminator(Terminator::ReturnVoid);
        self.builder.set_data(self.data.clone());
        self.builder.build_module()
    }

    fn lower_assignment(&mut self, assignment: &Assignment<'a>) {
        let value = self.lower_expression(&assignment.expression);
        self.bindings
            .insert(assignment.identifier.name.clone(), value);
    }

    fn lower_expression(&mut self, expr: &Expression<'a>) -> Value {
        match expr {
            Expression::Number(num) => self.lower_number(num),
            Expression::Identifier(ident) => self.lower_identifier(ident),
            Expression::FunctionCall(call) => self.lower_function_call(call),
            Expression::FunctionDefinition(_) => {
                panic!("Function definitions not yet implemented in lowering")
            }
            Expression::Block(block) => self.lower_block(block),
        }
    }

    fn lower_number(&mut self, num: &Number<'a>) -> Value {
        let var = self.builder.push_variable();
        self.builder.load_const(var, num.value as u64);
        var
    }

    fn lower_identifier(&mut self, ident: &Identifier<'a>) -> Value {
        *self
            .bindings
            .get(&ident.name)
            .unwrap_or_else(|| panic!("Undefined identifier: {}", ident.name))
    }

    fn lower_function_call(&mut self, call: &FunctionCall<'a>) -> Value {
        match call.function_name.name.as_str() {
            "write" => self.lower_write_syscall(call),
            "exit" => self.lower_exit_syscall(call),
            "string_literal" => self.lower_string_literal(call),
            _ => panic!("Unknown function: {}", call.function_name.name),
        }
    }

    fn lower_write_syscall(&mut self, call: &FunctionCall<'a>) -> Value {
        assert_eq!(call.arguments.len(), 3, "write expects 3 arguments");

        let fd = self.lower_expression(&call.arguments[0]);
        let buf = self.lower_expression(&call.arguments[1]);
        let len = self.lower_expression(&call.arguments[2]);

        let write_syscall = self.builder.push_variable();
        self.builder.load_const(write_syscall, SYS_WRITE);

        self.builder
            .build_syscall(vec![write_syscall, fd, buf, len]);

        write_syscall
    }

    fn lower_exit_syscall(&mut self, call: &FunctionCall<'a>) -> Value {
        assert_eq!(call.arguments.len(), 1, "exit expects 1 argument");

        let exit_code = self.lower_expression(&call.arguments[0]);

        let exit_syscall = self.builder.push_variable();
        self.builder.load_const(exit_syscall, SYS_EXIT);

        self.builder.build_syscall(vec![exit_syscall, exit_code]);

        exit_syscall
    }

    fn lower_string_literal(&mut self, call: &FunctionCall<'a>) -> Value {
        assert_eq!(
            call.arguments.len(),
            1,
            "string_literal expects 1 argument"
        );

        if let Expression::Identifier(ident) = &call.arguments[0] {
            let string_data = ident.name.as_bytes();
            let offset = self.data.len();
            self.data.extend_from_slice(string_data);

            let var = self.builder.push_variable();
            self.builder.load_data_addr(var, offset);
            var
        } else {
            panic!("string_literal expects identifier argument");
        }
    }

    fn lower_block(&mut self, block: &Block<'a>) -> Value {
        for assignment in &block.assignments {
            self.lower_assignment(assignment);
        }
        self.lower_expression(&block.expression)
    }
}

impl Default for AstLowering<'_> {
    fn default() -> Self {
        Self::new()
    }
}
