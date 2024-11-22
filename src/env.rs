use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

use crate::ast::*;

#[derive(Debug, Clone)]
pub struct Env {
    vars: HashMap<String, Value>,
    outer: Option<Rc<RefCell<Env>>>,
}

impl Env {
    pub fn new() -> Self {
        Env {
            vars: HashMap::new(),
            outer: None,
        }
    }

    pub fn extend(outer: Rc<RefCell<Env>>) -> Rc<RefCell<Env>> {
        Rc::new(RefCell::new(Env {
            vars: HashMap::new(),
            outer: Some(outer),
        }))
    }

    pub fn get_var(&self, id: &str) -> Option<Value> {
        if let Some(val) = self.vars.get(id) {
            Some(val.clone())
        } else if let Some(outer) = &self.outer {
            outer.borrow().get_var(id)
        } else {
            None
        }
    }

    pub fn set_var(&mut self, id: String, val: Value) {
        self.vars.insert(id, val);
    }
}

#[derive(Debug, Clone)]
pub enum Value {
    Num(i64),
    Bool(bool),
    Closure(Closure),
}

impl Value {
    pub fn to_bool(&self) -> bool {
        match self {
            Value::Num(val) => *val != 0,
            Value::Bool(val) => *val,
            _ => false,
        }
    }

    pub fn to_num(&self) -> i64 {
        match self {
            Value::Num(val) => *val,
            Value::Bool(val) => *val as i64,
            _ => 0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Closure { 
    pub params: Vec<String>, 
    pub body: Box<Exp>, 
    pub env: Rc<RefCell<Env>>
}

impl Closure {
    pub fn new(params: Vec<String>, body: Box<Exp>, env: Rc<RefCell<Env>>) -> Self {
        Closure { params, body, env }
    }
}