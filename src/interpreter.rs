use std::fmt::{self, Display, Formatter};
use std::collections::HashMap;

use crate::ast::*;

#[derive(Clone)]
struct Env {
    vars: HashMap<String, Value>,
    outer: Option<Box<Env>>,
}

impl Env {
    fn new() -> Self {
        Env {
            vars: HashMap::new(),
            outer: None,
        }
    }

    fn extend(&self) -> Env {
        Env {
            vars: HashMap::new(),
            outer: Some(Box::new(self.clone())),
        }
    }

    fn get_var(&self, id: &str) -> Option<Value> {
        match self.vars.get(id) {
            Some(val) => Some(val.clone()),
            None => match &self.outer { // Attention here
                Some(outer) => outer.get_var(id),
                None => None,
            },
        }
    }

    fn set_var(&mut self, id: String, val: Value) {
        self.vars.insert(id, val);
    }

}

#[derive(Debug, Clone)]
enum Value {
    Num(i64),
    Bool(bool),
    FunExp(Box<Exp>),
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Value::Num(val) => write!(f, "{}", val),
            Value::Bool(val) => write!(f, "{}", val),
            Value::FunExp(exp) => write!(f, "{:?}", exp),
        }
    }
}

impl Value {
    fn to_bool(&self) -> bool {
        match self {
            Value::Num(val) => *val != 0,
            Value::Bool(val) => *val,
            _ => false,
        }
    }

    fn to_num(&self) -> i64 {
        match self {
            Value::Num(val) => *val,
            Value::Bool(val) => *val as i64,
            _ => 0,
        }
    }

    fn to_fun(&self) -> Box<Exp> {
        match self {
            Value::FunExp(exp) => exp.clone(),
            _ => panic!("Expected function"),
        }
    }
}

pub fn run(program: Program) {
    let mut env = Env::new();
    for stmt in program.stmts {
        match stmt {
            Stmt::ExpStmt { exp } => {
                let _ = eval_exp(exp, &mut env);
            }
            Stmt::DefStmt { id, exp } => {
                let var_name = match id {
                    Exp::Id { val } => val,
                    _ => panic!("Expected identifier")
                };
                env.set_var(var_name, Value::FunExp(Box::new(exp)));
            }
            Stmt::PrintStmt { exp, print_type } => {
                let val = eval_exp(exp, &mut env).unwrap();
                match print_type {
                    PrintType::PrintNum => println!("{}", val.to_num()),
                    PrintType::PrintBool => println!("{}", if val.to_bool() { "#t" } else { "#f" }),
                }
            }
        }
    }
}

fn eval_exp(exp: Exp, env: &mut Env) -> Result<Value, String> {
    match exp {
        Exp::Bool { val } => Ok(Value::Bool(val)),
        Exp::Num  { val } => Ok(Value::Num(val)),
        Exp::Id   { val } => match env.get_var(&val) {
            Some(Value::FunExp(exp)) => eval_exp(*exp, env),
            Some(Value::Num(val))    => Ok(Value::Num(val)),
            Some(Value::Bool(val))   => Ok(Value::Bool(val)),
            None => panic!("Variable {} not found", val),
        },
        Exp::NumExp { op, args } => {
            let args = args.iter()
                .map(|arg| eval_exp(*arg.clone(), env).unwrap().to_num())
                .collect::<Vec<i64>>();
            match op {
                NumOp::Plus     => Ok(Value::Num(args.iter().sum())),
                NumOp::Minus    => Ok(Value::Num(args[0] - args[1])),
                NumOp::Multiply => Ok(Value::Num(args.iter().product())),
                NumOp::Divide   => Ok(Value::Num(args[0] / args[1])),
                NumOp::Modulus  => Ok(Value::Num(args[0] % args[1])),
                NumOp::Greater  => Ok(Value::Bool(args[0] > args[1])),
                NumOp::Smaller  => Ok(Value::Bool(args[0] < args[1])),
                NumOp::Equal    => Ok(Value::Bool(args[0] == args[1])),
            }
        }
        Exp::LogicalExp { op, args } => {
            let args = args.iter()
                .map(|arg| eval_exp(*arg.clone(), env).unwrap().to_bool())
                .collect::<Vec<bool>>();
            match op {
                LogicalOp::And => Ok(Value::Bool(args.iter().all(|&x| x))),
                LogicalOp::Or  => Ok(Value::Bool(args.iter().any(|&x| x))),
                LogicalOp::Not => Ok(Value::Bool(!args[0])),
            }
        },
        Exp::IfExp { cond_exp, then_exp, else_exp } => {
            if eval_exp(*cond_exp, env).unwrap().to_bool() {
                eval_exp(*then_exp, env)
            } else {
                eval_exp(*else_exp, env)
            }
        },
        Exp::FunExp { params, body } => {
            Ok(Value::FunExp(Box::new(Exp::FunExp { params, body })))
        },
        Exp::FunCall { func, args } => {
            let fun_exp = eval_exp(*func, env).unwrap().to_fun();
            if let Exp::FunExp { params, body } = *fun_exp {
                let mut new_env = env.extend();
                for (param, arg) in params.iter().zip(args.iter()) {
                    let val = eval_exp(*arg.clone(), env).unwrap();
                    new_env.set_var(param.to_string(), val);
                }
                eval_exp(*body, &mut new_env)
            } else {
                panic!("Expected function expression");
            }
        }
    }
}