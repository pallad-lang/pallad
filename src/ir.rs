#[derive(Debug, Clone)]
pub enum Instr {
    LoadInt(i64),
    LoadFloat(f64),
    LoadVar(String),
    StoreVar(String),
    Add,
    Sub,
    Mul,
    Div,
    IntDiv,
    Mod,
    CallBuiltin {
        name: String,
        argc: usize,
    },
    Pop,
}
