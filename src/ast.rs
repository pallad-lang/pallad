#[derive(Debug, Clone)]
pub enum Expr {
    None {
        line: usize,
    },
    Bool {
        value: bool,
        line: usize,
    },
    Int {
        value: i64,
        line: usize,
    },
    Float {
        value: f64,
        line: usize,
    },
    Str {
        value: String,
        line: usize,
    },
    Var {
        name: String,
        line: usize,
    },
    Binary {
        left: Box<Expr>,
        op: BinOp,
        right: Box<Expr>,
        line: usize,
    },
    Unary {
        op: UnOp,
        expr: Box<Expr>,
        line: usize,
    },
    Call {
        name: String,
        args: Vec<Expr>,
        line: usize,
    },
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Let {
        name: String,
        expr: Expr,
        line: usize,
    },
    Set {
        name: String,
        expr: Expr,
        line: usize,
    },
    Expr {
        expr: Expr,
        line: usize,
    },
}

#[derive(Debug, Clone)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    IntDiv,
    Mod,
    Pow,
    And,
    Or,
}

#[derive(Debug, Clone)]
pub enum UnOp {
    Neg,
    Not,
}
