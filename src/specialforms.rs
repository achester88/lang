use std::collections::HashMap;

#[path = "parser.rs"]
pub mod parser;

#[path = "expr.rs"]
pub mod expr;

use expr::Expr;

pub struct Specialforms<'a> {
  pub map: HashMap<&'a str, fn(&Box<Option<Vec<Expr>>>, &HashMap<String, String>) -> String>
}

impl Specialforms<'_> {

  pub fn out(args: &Box<Option<Vec<Expr>>>, scope: &HashMap<String, String>) -> String {
    if args.as_ref().as_ref().unwrap().len() != (1 as usize) {
      println!("Incorrect number of args at out");
      panic!();
    }
    let elm_expr = &args.as_ref().as_ref().unwrap()[0];
    println!("{}", elm_expr.value.as_ref().as_ref().unwrap());
    return String::from("true");
  }

  pub fn get(&self, s: &str) -> fn(&Box<Option<Vec<Expr>>>, &HashMap<String, String>) -> String {
    match self.map.get(s) {
            Some(f) => return *f,
            None => panic!(),
        };
  }
  
  pub fn new() -> Self {
    let mut temp: HashMap<_, fn(&Box<Option<Vec<Expr>>>, &HashMap<String, String>) -> String> = HashMap::new();
    temp.insert("out", Specialforms::out);
    return Self {map: temp};
  }
}

