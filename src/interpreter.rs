use std::rc::Rc;
use std::cell::RefCell;
use std::result::Result;

use crate::ast::*;
use crate::env::*;

pub fn run(program: Program) -> Result<(), String> {
    let env = Rc::new(RefCell::new(Env::new()));
    for stmt in program.stmts {
        eval_stmt(stmt, env.clone())?;
    }
    Ok(())
}

fn eval_stmt(stmt: Stmt, env: Rc<RefCell<Env>>) -> Result<(), String> {
    match stmt {
        Stmt::ExpStmt { exp } => {
            eval_exp(exp, env)?;
        }
        Stmt::DefStmt { id, exp } => {
            let id_str = id.to_string();
            let val = eval_exp(exp, env.clone())?;
            env.borrow_mut().set_var(id_str, val);
        }
        Stmt::PrintStmt { exp, print_type } => {
            let val = eval_exp(exp, env)?;
            match print_type {
                PrintType::PrintNum => println!("{}", val.to_num()?),
                PrintType::PrintBool => println!("{}", if val.to_bool()? { "#t" } else { "#f" }),
            }
        }
    }
    Ok(())
}

fn eval_exp(exp: Exp, env: Rc<RefCell<Env>>) -> Result<Value, String> {
    match exp {
        Exp::Bool(val) => Ok(Value::Bool(val)),
        Exp::Num(val) => Ok(Value::Num(val)),
        Exp::Id(val) => match env.borrow().get_var(&val) {
            Some(val) => Ok(val),
            None => panic!("Variable {} not found", val),
        },
        Exp::NumExp { op, args } => {
            let args = args.iter()
                .map(|arg| eval_exp(*arg.clone(), env.clone())?.to_num())
                .collect::<Result<Vec<i64>, String>>()?;
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
                .map(|arg| eval_exp(*arg.clone(), env.clone())?.to_bool())
                .collect::<Result<Vec<bool>, String>>()?;
            match op {
                LogicalOp::And => Ok(Value::Bool(args.iter().all(|&x| x))),
                LogicalOp::Or  => Ok(Value::Bool(args.iter().any(|&x| x))),
                LogicalOp::Not => Ok(Value::Bool(!args[0])),
            }
        },
        Exp::IfExp { cond_exp, then_exp, else_exp } => {
            if eval_exp(*cond_exp, env.clone())?.to_bool()? {
                eval_exp(*then_exp, env.clone())
            } else {
                eval_exp(*else_exp, env.clone())
            }
        },
        Exp::FunExp { params, def_stmts, body } => {
            let new_env = Env::extend(env.clone());
            for stmt in def_stmts {
                eval_stmt(stmt, new_env.clone())?;
            }
            Ok(Value::Closure(Closure::new(
                params.iter().map(|param| param.to_string()).collect(), 
                body, 
                new_env
            )))
        },
        Exp::FunCall { func, args } => {
            let fun_exp = eval_exp(*func, env.clone())?;
            match fun_exp {
                Value::Closure(closure) => {
                    let new_env = Env::extend(closure.env.clone());
                    for (param, arg) in closure.params.iter().zip(args) {
                        let arg_val = eval_exp(*arg, env.clone())?;
                        new_env.borrow_mut().set_var(param.to_string(), arg_val);
                    }
                    eval_exp(*closure.body.clone(), new_env)
                },
                _ => panic!("Expected a function"),
            }
        }
    }
}