pub mod instruction;

pub use instruction::{Condition, Instruction, Register, generate_elf_from_ir};

#[cfg(test)]
mod tests {
    use crate::ast::*;
    use crate::ssa::{
        ModuleBuilder, STDOUT, SYS_EXIT, SYS_WRITE, Terminator, lower,
    };

    #[test]
    fn test_hello_world_assembly() {
        let mut builder = ModuleBuilder::default();

        let block = builder.push_bb();
        builder.switch_to_block(block);

        let fd = builder.push_variable();
        builder.load_const(fd, STDOUT);

        let buf = builder.push_variable();
        builder.load_data_addr(buf, 0);

        let len = builder.push_variable();
        builder.load_const(len, 12);

        let write_syscall = builder.push_variable();
        builder.load_const(write_syscall, SYS_WRITE);

        builder.build_syscall(vec![write_syscall, fd, buf, len]);

        let exit_code = builder.push_variable();
        builder.load_const(exit_code, 0);

        let exit_syscall = builder.push_variable();
        builder.load_const(exit_syscall, SYS_EXIT);

        builder.build_syscall(vec![exit_syscall, exit_code]);

        builder.set_terminator(Terminator::ReturnVoid);
        builder.set_data(b"Hello World\n".to_vec());

        let module = builder.build_module();
        let ir_program = lower(&module);
        let binary = super::generate_elf_from_ir(&ir_program);

        assert!(binary.starts_with(b"\x7fELF"));
        assert_eq!(binary.len(), 164);
    }

    #[test]
    fn test_hello_world_from_ast() {
        let source = "hello world";
        let span = Span::new(source);

        let program = Program {
            assignments: vec![Assignment {
                identifier: Identifier {
                    name: "main".to_string(),
                    span,
                },
                expression: Box::new(Expression::FunctionDefinition(
                    FunctionDefinition {
                        parameters: vec![],
                        body: Box::new(Expression::Block(Block {
                            assignments: vec![
                                Assignment {
                                    identifier: Identifier {
                                        name: "fd".to_string(),
                                        span,
                                    },
                                    expression: Box::new(Expression::Number(
                                        Number { value: 1, span },
                                    )),
                                    span,
                                },
                                Assignment {
                                    identifier: Identifier {
                                        name: "msg".to_string(),
                                        span,
                                    },
                                    expression: Box::new(
                                        Expression::FunctionCall(
                                            FunctionCall {
                                                function_name: Identifier {
                                                    name: "string_literal"
                                                        .to_string(),
                                                    span,
                                                },
                                                arguments: vec![
                                                    Expression::Identifier(
                                                        Identifier {
                                                            name:
                                                                "Hello World\n"
                                                                    .to_string(),
                                                            span,
                                                        },
                                                    ),
                                                ],
                                                span,
                                            },
                                        ),
                                    ),
                                    span,
                                },
                                Assignment {
                                    identifier: Identifier {
                                        name: "len".to_string(),
                                        span,
                                    },
                                    expression: Box::new(Expression::Number(
                                        Number { value: 12, span },
                                    )),
                                    span,
                                },
                                Assignment {
                                    identifier: Identifier {
                                        name: "_".to_string(),
                                        span,
                                    },
                                    expression: Box::new(
                                        Expression::FunctionCall(
                                            FunctionCall {
                                                function_name: Identifier {
                                                    name: "write".to_string(),
                                                    span,
                                                },
                                                arguments: vec![
                                                    Expression::Identifier(
                                                        Identifier {
                                                            name: "fd"
                                                                .to_string(),
                                                            span,
                                                        },
                                                    ),
                                                    Expression::Identifier(
                                                        Identifier {
                                                            name: "msg"
                                                                .to_string(),
                                                            span,
                                                        },
                                                    ),
                                                    Expression::Identifier(
                                                        Identifier {
                                                            name: "len"
                                                                .to_string(),
                                                            span,
                                                        },
                                                    ),
                                                ],
                                                span,
                                            },
                                        ),
                                    ),
                                    span,
                                },
                            ],
                            expression: Box::new(Expression::FunctionCall(
                                FunctionCall {
                                    function_name: Identifier {
                                        name: "exit".to_string(),
                                        span,
                                    },
                                    arguments: vec![Expression::Number(
                                        Number { value: 0, span },
                                    )],
                                    span,
                                },
                            )),
                            span,
                        })),
                        span,
                    },
                )),
                span,
            }],
            span,
        };

        println!("AST:\n{}", program);

        let lowering = AstLowering::new();
        let module = lowering.lower_program(&program);
        let ir_program = lower(&module);
        let binary = super::generate_elf_from_ir(&ir_program);

        assert!(binary.starts_with(b"\x7fELF"));
        assert_eq!(binary.len(), 164);
    }
}
