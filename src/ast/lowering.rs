use crate::ast::types::{
    Assignment, BinaryOperator, Block, Expression, FunctionCall,
    FunctionDefinition, Identifier, Number, Program,
};
use crate::ssa::{
    Module, ModuleBuilder, SYS_EXIT, SYS_WRITE, Terminator, Value,
    module::Function,
};
use std::collections::HashMap;

pub struct AstLowering<'a> {
    builder: ModuleBuilder,
    bindings: HashMap<String, Value>,
    functions: HashMap<String, (Vec<String>, Expression<'a>)>,
    data: Vec<u8>,
    _phantom: std::marker::PhantomData<&'a ()>,
}

impl<'a> AstLowering<'a> {
    pub fn new() -> Self {
        Self {
            builder: ModuleBuilder::default(),
            bindings: HashMap::new(),
            functions: HashMap::new(),
            data: Vec::new(),
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn lower_program(mut self, program: &Program<'a>) -> Module {
        for assignment in &program.assignments {
            if let Expression::FunctionDefinition(func_def) =
                &*assignment.expression
            {
                self.functions.insert(
                    assignment.identifier.name.clone(),
                    (
                        func_def
                            .parameters
                            .iter()
                            .map(|p| p.name.clone())
                            .collect(),
                        (*func_def.body).clone(),
                    ),
                );
            }
        }

        let functions_clone = self.functions.clone();

        let block = self.builder.push_bb();
        self.builder.switch_to_block(block);

        for assignment in &program.assignments {
            self.lower_assignment(assignment);
        }

        self.builder.set_terminator(Terminator::ReturnVoid);
        let mut module = self.builder.build_module();

        let data_clone = self.data.clone();
        module.data = data_clone.clone();

        for (func_name, (params, body)) in &functions_clone {
            let func = Self::lower_function_static(
                func_name.clone(),
                params.clone(),
                body,
                &functions_clone,
                &data_clone,
            );
            module.functions.insert(func_name.clone(), func);
        }

        module
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
            Expression::FunctionDefinition(func_def) => {
                self.lower_function_definition(func_def)
            }
            Expression::Block(block) => self.lower_block(block),
            Expression::BinaryOp(binop) => self.lower_binary_op(binop),
            Expression::Conditional(cond) => self.lower_conditional(cond),
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
            name => {
                if self.functions.contains_key(name) {
                    let args: Vec<Value> = call
                        .arguments
                        .iter()
                        .map(|arg| self.lower_expression(arg))
                        .collect();
                    let result = self.builder.push_variable();
                    self.builder.build_call(result, name.to_string(), args);
                    result
                } else {
                    panic!("Unknown function: {}", name)
                }
            }
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

    fn lower_function_definition(
        &mut self,
        func_def: &FunctionDefinition<'a>,
    ) -> Value {
        if func_def.parameters.is_empty() {
            self.lower_expression(&func_def.body)
        } else {
            let dummy = self.builder.push_variable();
            self.builder.load_const(dummy, 0);
            dummy
        }
    }

    fn lower_binary_op(&mut self, binop: &crate::ast::BinaryOp<'a>) -> Value {
        let left = self.lower_expression(&binop.left);
        let right = self.lower_expression(&binop.right);
        let result = self.builder.push_variable();

        let op = match binop.operator {
            BinaryOperator::Add => crate::ssa::BinaryOp::Add,
            BinaryOperator::Subtract => crate::ssa::BinaryOp::Sub,
            BinaryOperator::Multiply => crate::ssa::BinaryOp::Mul,
            BinaryOperator::Divide => crate::ssa::BinaryOp::Div,
            BinaryOperator::Eq => crate::ssa::BinaryOp::Eq,
            BinaryOperator::NotEq => crate::ssa::BinaryOp::NotEq,
            BinaryOperator::Lt => crate::ssa::BinaryOp::Lt,
            BinaryOperator::Le => crate::ssa::BinaryOp::Le,
            BinaryOperator::Gt => crate::ssa::BinaryOp::Gt,
            BinaryOperator::Ge => crate::ssa::BinaryOp::Ge,
        };

        self.builder.build_binop(result, left, right, op);
        result
    }

    fn lower_conditional(
        &mut self,
        cond: &crate::ast::Conditional<'a>,
    ) -> Value {
        let condition_value = self.lower_expression(&cond.condition);

        let then_block = self.builder.push_bb();
        let else_block = self.builder.push_bb();
        let merge_block = self.builder.push_bb();

        let result_var = self.builder.push_variable();

        self.builder.build_branch(
            condition_value,
            then_block,
            vec![],
            else_block,
            vec![],
        );

        self.builder.switch_to_block(then_block);
        let then_value = self.lower_expression(&cond.then_expr);
        self.builder.build_jump(merge_block, vec![then_value]);

        self.builder.switch_to_block(else_block);
        let else_value = self.lower_expression(&cond.else_expr);
        self.builder.build_jump(merge_block, vec![else_value]);

        self.builder.switch_to_block(merge_block);
        self.builder.add_block_param(merge_block, result_var);

        result_var
    }

    fn lower_function_static(
        name: String,
        params: Vec<String>,
        body: &Expression<'a>,
        functions: &HashMap<String, (Vec<String>, Expression<'a>)>,
        data: &Vec<u8>,
    ) -> Function {
        let mut func_builder = ModuleBuilder::default();
        let mut func_bindings = HashMap::new();

        let entry_block = func_builder.push_bb();

        for (i, param_name) in params.iter().enumerate() {
            let param_value = func_builder.push_variable();
            func_builder.add_block_param(entry_block, param_value);
            func_bindings.insert(param_name.clone(), param_value);
        }

        func_builder.switch_to_block(entry_block);

        let mut func_lowering = AstLowering {
            builder: func_builder,
            bindings: func_bindings,
            functions: functions.clone(),
            data: data.clone(),
            _phantom: std::marker::PhantomData,
        };

        let return_value = func_lowering.lower_expression(body);
        func_lowering
            .builder
            .set_terminator(Terminator::Return(return_value));

        let module = func_lowering.builder.build_module();

        Function {
            name,
            params,
            blocks: module.blocks,
        }
    }
}

impl Default for AstLowering<'_> {
    fn default() -> Self {
        Self::new()
    }
}
