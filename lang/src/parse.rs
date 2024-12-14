pub use ast::parse;

use crate::ast::{Ast, Entrypoint, Expr};

peg::parser! {
    grammar ast() for str {
        pub rule parse() -> Ast
            = entrypoint:entrypoint() { Ast { entrypoint } }


        rule whitespace() = quiet!{ [' ' | '\n' | '\t'] }

        rule _ = quiet! { whitespace()* }
        rule __ = whitespace()+

        rule alphanumeric() = ['a'..='z' | 'A'..='Z' | '0'..='9']

        pub rule expr() -> Expr
            = _
            "noop"
            { Expr::Noop }

        pub rule entrypoint() -> Entrypoint
            = _ "entrypoint" _ "{"
                _ e:(exprs:expr() ** __ { Entrypoint { exprs } }) _
            "}" _ { e }

    }
}
