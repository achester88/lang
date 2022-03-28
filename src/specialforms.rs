use std::collections::HashMap;

#[path = "parser.rs"]
pub mod parser;

use parser::{Expr};

pub struct Specialforms<'a> {
  pub map: HashMap<&'a str, fn(Box<Option<Vec<Expr>>>, HashMap<String, String>) -> String>
}

impl Specialforms<'_> {

  pub fn out(args: Box<Option<Vec<Expr>>>, scope: HashMap<String, String>) -> String {
    println!("{}", args.unwrap()[0].value.unwrap());
    return String::from("true");
  }

  pub fn get(&self, s: &str) -> fn(Box<Option<Vec<Expr>>>, HashMap<String, String>) -> String {
    match self.map.get(s) {
            Some(f) => return *f,
            None => panic!(),
        };
  }
  
  pub fn new() -> Self {
    let mut temp: HashMap<_, fn(Box<Option<Vec<Expr>>>, HashMap<String, String>) -> String> = HashMap::new();
    temp.insert("out", Specialforms::out);
    return Self {map: temp};
  }
}

