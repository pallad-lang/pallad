#[derive(Debug, Clone)]
pub enum Instr {
    LoadNone {
        line: usize,
    },
    LoadBool {
        value: bool,
        line: usize,
    },
    LoadInt {
        value: i64,
        line: usize,
    },
    LoadFloat {
        value: f64,
        line: usize,
    },
    LoadStr {
        value: String,
        line: usize,
    },
    LoadVar {
        name: String,
        line: usize,
    },
    StoreVar {
        name: String,
        line: usize,
    },
    SetVar {
        name: String,
        line: usize,
    },
    Add {
        line: usize,
    },
    Sub {
        line: usize,
    },
    Mul {
        line: usize,
    },
    Div {
        line: usize,
    },
    IntDiv {
        line: usize,
    },
    Mod {
        line: usize,
    },
    Pow {
        line: usize,
    },
    And {
        line: usize,
    },
    Or {
        line: usize,
    },
    Neg {
        line: usize,
    },
    Not {
        line: usize,
    },
    CallBuiltin {
        name: String,
        argc: usize,
        line: usize,
    },
    Pop {
        line: usize,
    },
}
