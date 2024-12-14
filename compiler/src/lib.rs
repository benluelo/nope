use bytecode::{Bytecode, Opcode};
use lang::ast::{Ast, Expr};

pub fn compile(ast: Ast) -> Bytecode {
    ast.entrypoint
        .exprs
        .into_iter()
        .fold(Bytecode::new(), |mut bc, expr| {
            for opcode in compile_expr(expr) {
                bc.push(opcode);
            }

            bc
        })
}

fn compile_expr(expr: Expr) -> Vec<Opcode> {
    match expr {
        Expr::Noop => vec![Opcode::Noop],
    }
}
