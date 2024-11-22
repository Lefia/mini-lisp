use pest::{Parser as ParserTrait, iterators::Pair};
use pest_derive::Parser;

use crate::ast::*;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct Parser;

pub fn parse(input: &str) -> Result<Program, pest::error::Error<Rule>> {
    let mut pairs = Parser::parse(Rule::PROGRAM, input)?;
    let program = parse_program(pairs.next().unwrap())?;
    Ok(program)
}

fn parse_program(pair: Pair<Rule>) -> Result<Program, pest::error::Error<Rule>> {
    assert!(pair.as_rule() == Rule::PROGRAM);

    let stmts = pair.into_inner().map(|stmt| {
        parse_stmt(stmt)
    }).collect::<Result<Vec<Stmt>, _>>()?;
    
    Ok(Program{stmts})
}

fn parse_stmt(pair: Pair<Rule>) -> Result<Stmt, pest::error::Error<Rule>> {
    assert!(pair.as_rule() == Rule::STMT);

    let stmt = pair.into_inner().next().unwrap();
    match stmt.as_rule() {
        Rule::EXP => parse_exp_stmt(stmt),
        Rule::DEF_STMT => parse_def_stmt(stmt),
        Rule::PRINT_STMT => parse_print_stmt(stmt),
        _ => unreachable!()
    }
}

fn parse_exp_stmt(pair: Pair<Rule>) -> Result<Stmt, pest::error::Error<Rule>> {
    assert!(pair.as_rule() == Rule::EXP);

    let exp = parse_exp(pair)?;

    Ok(Stmt::ExpStmt{exp})
}

fn parse_def_stmt(pair: Pair<Rule>) -> Result<Stmt, pest::error::Error<Rule>>{
    assert!(pair.as_rule() == Rule::DEF_STMT);
    
    let mut inner = pair.into_inner();
    let id = parse_id(inner.next().unwrap())?;
    let exp = parse_exp(inner.next().unwrap())?;
    Ok(Stmt::DefStmt{id, exp})
    
}

fn parse_print_stmt(pair: Pair<Rule>) -> Result<Stmt, pest::error::Error<Rule>>{
    assert!(pair.as_rule() == Rule::PRINT_STMT);

    let print_type = if pair.as_str().contains("print-bool") {
        PrintType::PrintBool
    } else {
        PrintType::PrintNum
    };
    
    let inner = pair.into_inner().next().unwrap();
    let exp = parse_exp(inner)?;
    Ok(Stmt::PrintStmt{exp, print_type})
}

fn parse_exp(pair: Pair<Rule>) -> Result<Exp, pest::error::Error<Rule>> {
    assert!(pair.as_rule() == Rule::EXP);

    let exp = pair.into_inner().next().unwrap();
    match exp.as_rule() {
        Rule::bool => parse_bool(exp),
        Rule::number => parse_num(exp),
        Rule::id => parse_id(exp),
        Rule::NUM_OP => parse_num_exp(exp),
        Rule::LOGICAL_OP => parse_logical_exp(exp),
        Rule::FUN_EXP => parse_fun_exp(exp),
        Rule::FUN_CALL => parse_fun_call(exp),
        Rule::IF_EXP => parse_if_exp(exp),
        _ => unreachable!()
    }
}

fn parse_bool(pair: Pair<Rule>) -> Result<Exp, pest::error::Error<Rule>> {
    assert!(pair.as_rule() == Rule::bool);
    
    let val = match pair.as_str() {
        "#t" => true,
        "#f" => false,
        _ => unreachable!()
    };
    Ok(Exp::Bool(val))
}

fn parse_num(pair: Pair<Rule>) -> Result<Exp, pest::error::Error<Rule>> {
    assert!(pair.as_rule() == Rule::number);
    
    let val: i64 = pair.as_str().parse().unwrap();
    Ok(Exp::Num(val))
}

fn parse_id(string: Pair<Rule>) -> Result<Exp, pest::error::Error<Rule>> {
    let val = string.as_str().to_string();
    Ok(Exp::Id(val))
}

fn parse_num_exp(pair: Pair<Rule>) -> Result<Exp, pest::error::Error<Rule>> {
    assert!(pair.as_rule() == Rule::NUM_OP);
    
    let num_exp = pair.into_inner().next().unwrap();
    let op = match num_exp.as_rule() {
        Rule::PLUS => NumOp::Plus,
        Rule::MINUS => NumOp::Minus,
        Rule::MULTIPLY => NumOp::Multiply,
        Rule::DIVIDE => NumOp::Divide,
        Rule::MODULUS => NumOp::Modulus,
        Rule::GREATER => NumOp::Greater,
        Rule::SMALLER => NumOp::Smaller,
        Rule::EQUAL => NumOp::Equal,
        _ => unreachable!()
    };
    let args = num_exp.into_inner().map(|exp| {
        Box::new(parse_exp(exp).unwrap())
    }).collect();
    Ok(Exp::NumExp{op, args})
}

fn parse_logical_exp(pair: Pair<Rule>) -> Result<Exp, pest::error::Error<Rule>> {
    assert!(pair.as_rule() == Rule::LOGICAL_OP);
    
    let logical_exp = pair.into_inner().next().unwrap();
    let op = match logical_exp.as_rule() {
        Rule::AND_OP => LogicalOp::And,
        Rule::OR_OP  => LogicalOp::Or,
        Rule::NOT_OP => LogicalOp::Not,
        _ => unreachable!()
    };
    let args = logical_exp.into_inner().map(|exp| {
        Box::new(parse_exp(exp).unwrap())
    }).collect();
    Ok(Exp::LogicalExp{op, args})
}

fn parse_fun_exp(pair: Pair<Rule>) -> Result<Exp, pest::error::Error<Rule>> {
    assert!(pair.as_rule() == Rule::FUN_EXP);
    
    let mut fun_exp = pair.into_inner();

    let ids = fun_exp.next().unwrap();
    let params = ids.into_inner().map(|id| {
        parse_id(id).unwrap()
    }).collect();

    let mut stmts = Vec::new();
    let mut exp = None;

    let func_body = fun_exp.next().unwrap();

    for pair in func_body.into_inner() {
        match pair.as_rule() {
            Rule::DEF_STMT => {
                stmts.push(parse_def_stmt(pair)?);
            },
            Rule::EXP => {
                exp = Some(parse_exp(pair)?);
                break;
            },
            _ => unreachable!()
        }
    }

    let body = exp.unwrap();

    Ok(Exp::FunExp { params, def_stmts: stmts, body: Box::new(body) })
}

fn parse_fun_call(pair: Pair<Rule>) -> Result<Exp, pest::error::Error<Rule>> {
    assert!(pair.as_rule() == Rule::FUN_CALL);
    
    let mut fun_call = pair.into_inner();

    let first_exp = fun_call.next().unwrap();
    let func = match first_exp.as_rule() {
        Rule::id => Box::new(parse_id(first_exp)?),
        Rule::FUN_EXP => Box::new(parse_fun_exp(first_exp)?),
        _ => unreachable!()
    };

    let args = fun_call.map(|exp| {
        Box::new(parse_exp(exp).unwrap())
    }).collect();

    Ok(Exp::FunCall{func, args})
}

fn parse_if_exp(pair: Pair<Rule>) -> Result<Exp, pest::error::Error<Rule>> {
    assert!(pair.as_rule() == Rule::IF_EXP);

    let mut if_exp = pair.into_inner();
    let cond_exp = Box::new(parse_exp(if_exp.next().unwrap())?);
    let then_exp = Box::new(parse_exp(if_exp.next().unwrap())?);
    let else_exp = Box::new(parse_exp(if_exp.next().unwrap())?);

    Ok(Exp::IfExp{cond_exp, then_exp, else_exp})
}
