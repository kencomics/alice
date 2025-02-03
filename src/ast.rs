pub struct Module {
    pub name: String,
    pub stmts: Vec<Stmt>,
}

pub enum Stmt {
    Alias(Alias),
}

pub struct Alias {
pub     name: String,
pub     value: Expr,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Expr {
    Int(i32),
}
