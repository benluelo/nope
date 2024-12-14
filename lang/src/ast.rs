pub struct Ast {
    pub entrypoint: Entrypoint,
}

pub struct Entrypoint {
    pub exprs: Vec<Expr>,
}

pub enum Expr {
    Noop,
}
