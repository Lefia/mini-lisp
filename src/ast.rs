#![allow(dead_code)]

#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    pub stmts: Vec<Stmt>
}

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    ExpStmt   { exp: Exp },
    PrintStmt { print_type: PrintType, exp: Exp },
    DefStmt   { id: Exp, exp: Exp },
}

#[derive(Debug, Clone, PartialEq)]
pub enum PrintType {
    PrintNum,
    PrintBool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Exp {
    Bool(bool),
    Num(i64),
    Id(String),
    NumExp     { op: NumOp, args: Vec<Box<Exp>>      },
    LogicalExp { op: LogicalOp, args: Vec<Box<Exp>>  },
    FunExp     { params: Vec<Exp>, def_stmts: Vec<Stmt>, body: Box<Exp> },
    FunCall    { func: Box<Exp>, args: Vec<Box<Exp>> },
    IfExp      { cond_exp: Box<Exp>, then_exp: Box<Exp>, else_exp: Box<Exp> },
}

impl Exp {
    pub fn to_string(&self) -> String {
        match self {
            Exp::Bool(val) => val.to_string(),
            Exp::Num(val) => val.to_string(),
            Exp::Id(val) => val.to_string(),
            _ => unimplemented!()
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum NumOp {
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulus,
    Greater,
    Smaller,
    Equal,
}

#[derive(Debug, Clone, PartialEq)]
pub enum LogicalOp {
    And,
    Or,
    Not,
}