use std::cell::RefCell;
use std::io::Write;
use std::rc::Rc;
use std::result::Result;

use crate::ast::*;
use crate::env::*;

pub fn run<W: Write>(program: Program, writer: &mut W) -> Result<(), (String, String)> {
    let env = Rc::new(RefCell::new(Env::new()));
    for stmt in program.stmts {
        eval_stmt(stmt, env.clone(), writer)?;
    }
    Ok(())
}

fn eval_stmt<W: Write>(stmt: Stmt, env: Rc<RefCell<Env>>, writer: &mut W) -> Result<(), (String, String)> {
    match stmt {
        Stmt::ExpStmt { exp } => {
            eval_exp(exp, env.clone(), writer)?;
        }
        Stmt::DefStmt { id, exp } => {
            let id_str = id.to_string();
            let val = eval_exp(exp, env.clone(), writer)?;
            env.borrow_mut().set_var(id_str, val);
        }
        Stmt::PrintStmt { exp, print_type } => {
            let val = eval_exp(exp, env.clone(), writer)?;
            match print_type {
                PrintType::PrintNum => {
                    writeln!(writer, "{}", val.to_num()?).unwrap();
                }
                PrintType::PrintBool => {
                    writeln!(writer, "{}", if val.to_bool()? { "#t" } else { "#f" }).unwrap();
                }
            };
        }
    }
    Ok(())
}

fn eval_exp<W: Write>(exp: Exp, env: Rc<RefCell<Env>>, writer: &mut W) -> Result<Value, (String, String)> {
    match exp {
        Exp::Bool(val) => Ok(Value::Bool(val)),
        Exp::Num(val) => Ok(Value::Num(val)),
        Exp::Id(val) => match env.borrow().get_var(&val) {
            Some(val) => Ok(val),
            None => Err(("syntax error".to_string(), format!("variable '{}' not found", val))),
        },
        Exp::NumExp { op, args } => {
            let args = args
                .iter()
                .map(|arg| eval_exp(*arg.clone(), env.clone(), writer)?.to_num())
                .collect::<Result<Vec<i64>, (String, String)>>()?;
            match op {
                NumOp::Plus => Ok(Value::Num(args.iter().sum())),
                NumOp::Minus => Ok(Value::Num(args[0] - args[1])),
                NumOp::Multiply => Ok(Value::Num(args.iter().product())),
                NumOp::Divide => Ok(Value::Num(args[0] / args[1])),
                NumOp::Modulus => Ok(Value::Num(args[0] % args[1])),
                NumOp::Greater => Ok(Value::Bool(args[0] > args[1])),
                NumOp::Smaller => Ok(Value::Bool(args[0] < args[1])),
                NumOp::Equal => Ok(Value::Bool(args[0] == args[1])),
            }
        }
        Exp::LogicalExp { op, args } => {
            let args = args
                .iter()
                .map(|arg| eval_exp(*arg.clone(), env.clone(), writer)?.to_bool())
                .collect::<Result<Vec<bool>, (String, String)>>()?;
            match op {
                LogicalOp::And => Ok(Value::Bool(args.iter().all(|&x| x))),
                LogicalOp::Or => Ok(Value::Bool(args.iter().any(|&x| x))),
                LogicalOp::Not => Ok(Value::Bool(!args[0])),
            }
        }
        Exp::IfExp {
            cond_exp,
            then_exp,
            else_exp,
        } => {
            if eval_exp(*cond_exp, env.clone(), writer)?.to_bool()? {
                eval_exp(*then_exp, env.clone(), writer)
            } else {
                eval_exp(*else_exp, env.clone(), writer)
            }
        }
        Exp::FunExp {
            params,
            def_stmts,
            body,
        } => {
            let new_env = Env::extend(env.clone());
            for stmt in def_stmts {
                eval_stmt(stmt, new_env.clone(), writer)?;
            }
            Ok(Value::Closure(Closure::new(
                params.iter().map(|param| param.to_string()).collect(),
                body,
                new_env,
            )))
        }
        Exp::FunCall { func, args } => {
            let fun_exp = eval_exp(*func, env.clone(), writer)?;
            match fun_exp {
                Value::Closure(closure) => {
                    let new_env = Env::extend(closure.env.clone());
                    for (param, arg) in closure.params.iter().zip(args) {
                        let arg_val = eval_exp(*arg, env.clone(), writer)?;
                        new_env.borrow_mut().set_var(param.to_string(), arg_val);
                    }
                    eval_exp(*closure.body.clone(), new_env, writer)
                }
                _ => unreachable!()
            }
        }
    }
}
