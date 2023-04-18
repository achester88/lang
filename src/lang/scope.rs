use std::collections::HashMap;
use crate::expr::*;

pub struct Scope {
  vmap: HashMap<String, Value>,
  fnmap: HashMap<String, EFn>,
}

impl Scope {
  pub fn new() -> Self {
    
    Scope {
      vmap: HashMap::new(),
      fnmap: HashMap::new()
    }   
  }

  pub fn findget(&mut self, val: &str) -> Option<&Value> {
    if self.vmap.contains_key(val) {
        Some(self.vmap.get(val).unwrap())
        //return scope[expr.name];
      } else {
       return None;
      }
  }

  pub fn insert(&mut self, name: String, val: &Value) {
    self.vmap.insert(name, val.to_owned());
  }

  fn to_param(name_of: &Expr, type_of: &Expr) -> (String, Value) {
    let name = name_of.get_value();
    match type_of.get_value().to_string().as_str() {
      "int" => (name.to_string(), Value::Int(0)),
      "string" => (name.to_string(), Value::to_stringv("")),
      "bool" => (name.to_string(), Value::Bool(false)),
      _ => {
        println!("Unknown type {} at fn defintion", &type_of.get_value().to_string());
        panic!()
      }
    }
  }
  
  pub fn makefn(&mut self, name: String, stat: Expr, cond: Vec<Expr>) {
    if cond.len() % 2 != 0 {
      println!("Missing pair of args at fn {:?}", name);
      panic!();
    }
    let mut params: Vec<(String, Value)> = vec!();
    for pair in cond.chunks(2) {
      params.push(Scope::to_param(&pair[1], &pair[0]));
    }
    self.fnmap.insert(name, (stat, params));
  }

  pub fn isfn(&mut self, name: &str) -> Option<EFn> {
    if self.fnmap.contains_key(name) {
      return Some(self.fnmap.get(name).unwrap().to_owned());
    }
    return None;
  }
}