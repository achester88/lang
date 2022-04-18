use std::io;
use std::io::Write; // <--- bring flush() into scope

use std::collections::HashMap;
use std::any::type_name;

//use crate::parser;
use crate::expr;
use crate::evaluate;

//#[path = "parser.rs"]
//pub mod parser;

//#[path = "expr.rs"]
//pub mod expr;

//#[path = "evaluate.rs"]
//pub mod evaluate;
//use evaluate::*;
use expr::*;


pub struct Specialforms<'a> {
  pub map: HashMap<&'a str, fn(&mut evaluate::Evaluate<'_>, &Vec<Expr>, &mut HashMap<String, String>) -> String>
}

impl Specialforms<'_> {

//---------------------------------------------------------
//---------------------------------------------------------
  pub fn outln(eval: &mut evaluate::Evaluate<'_>, args: &Vec<Expr>, scope: &mut HashMap<String, String>) -> String {
    if args.len() != (1 as usize) {
      println!("Incorrect number of args at out");
      panic!();
    }
    //let elm_expr = &args.as_ref().as_ref().unwrap()[0];
    //let val = elm_expr.value.as_ref().as_ref().unwrap();
    let raw = eval.evaluate(args[0].clone(), scope);
    if raw.contains('"') {
      let mes = &raw[1..raw.len()-1];
      print!("{}", mes);
    } else {
      print!("{}", raw);
    }

    return String::from("true");
  }

  pub fn out(eval: &mut evaluate::Evaluate<'_>, args: &Vec<Expr>, scope: &mut HashMap<String, String>) -> String {
    if args.len() != (1 as usize) {
      println!("Incorrect number of args at out");
      panic!();
    }
    //let elm_expr = &args.as_ref().as_ref().unwrap()[0];
    //let val = elm_expr.value.as_ref().as_ref().unwrap();
    let raw = eval.evaluate(args[0].clone(), scope);
    if raw.contains('"') {
      let mes = &raw[1..raw.len()-1];
      print!("{}", mes);
    } else {
      print!("{}", raw);
    }
    io::stdout().flush().unwrap();
    return String::from("true");
  }

  pub fn fnif(eval: &mut evaluate::Evaluate<'_>, args: &Vec<Expr>, scope:   &mut HashMap<String, String>) -> String {
    if args.len() != (3 as usize) {
      println!("Incorrect number of args at if");
      panic!();
    } else if eval.evaluate(args[0].clone(), scope) != "false" {
      return  eval.evaluate(args[1].clone(), scope);
    } else {
      return  eval.evaluate(args[2].clone(), scope);
    }
  }

  pub fn fnwhile(eval: &mut evaluate::Evaluate<'_>, args: &Vec<Expr>, scope: &mut HashMap<String, String>) -> String {
    if args.len() != (2 as usize) {
      println!("Incorrect number of args at while");
      panic!();
    }
    while eval.evaluate(args[0].clone(), scope) != "false" {
      eval.evaluate(args[1].clone(), scope);
    }
    
    return String::from("false");
  }

  pub fn fndo(eval: &mut evaluate::Evaluate<'_>, args: &Vec<Expr>, scope: &mut HashMap<String, String>) -> String {
    let mut value = String::from("false");
    for arg in args {
      //println!("arg: {:#?}", arg);
      value = eval.evaluate(arg.clone(), scope);
    }
    return value;
  }

  pub fn fnint(eval: &mut evaluate::Evaluate<'_>, args: &Vec<Expr>, scope: &mut HashMap<String, String>) -> String {
     if args.len() != (2 as usize) || args[0].type_of != "word" {
      println!("Incorrect number of args at define");
      panic!();
    }
    let value = eval.evaluate(args[1].clone(), scope);
    let name = args[0].get_value();
    scope.insert(String::from(name), value.clone());
    return value;
  }

  pub fn fnbool(eval: &mut evaluate::Evaluate<'_>, args: &Vec<Expr>, scope: &mut HashMap<String, String>) -> String {
     if args.len() != (2 as usize) || args[0].type_of != "word" {
      println!("Incorrect number of args at define");
      panic!();
    }
    let value = eval.evaluate(args[1].clone(), scope);
    let name = args[0].get_value();
    scope.insert(String::from(name), value.clone());
    return value;
  }
  
  pub fn fnadd(eval: &mut evaluate::Evaluate<'_>, args: &Vec<Expr>, scope: &mut HashMap<String, String>) -> String {
    
    let (i1, i2) = Specialforms::check(eval, args, scope);
    return (i1 + i2).to_string();
  }
  
  pub fn fnsub(eval: &mut evaluate::Evaluate<'_>, args: &Vec<Expr>, scope: &mut HashMap<String, String>) -> String {
    
    let (i1, i2) = Specialforms::check(eval, args, scope);
    return (i1 - i2).to_string();
  }
  pub fn fnmut(eval: &mut evaluate::Evaluate<'_>, args: &Vec<Expr>, scope: &mut HashMap<String, String>) -> String {
    
    let (i1, i2) = Specialforms::check(eval, args, scope);
    return (i1 * i2).to_string();
  }
  pub fn fndiv(eval: &mut evaluate::Evaluate<'_>, args: &Vec<Expr>, scope: &mut HashMap<String, String>) -> String {
    
    let (i1, i2) = Specialforms::check(eval, args, scope);
    return (i1 / i2).to_string();
  }
  
  
  pub fn fneq(eval: &mut evaluate::Evaluate<'_>, args: &Vec<Expr>, scope: &mut HashMap<String, String>) -> String {
    
    let (i1, i2) = Specialforms::check(eval, args, scope);
    return (i1 == i2).to_string();
  }
  
  pub fn fngt(eval: &mut evaluate::Evaluate<'_>, args: &Vec<Expr>, scope: &mut HashMap<String, String>) -> String {             
    let (i1, i2) = Specialforms::check(eval, args, scope);
    return (i1 > i2).to_string();
  }
  
  pub fn fnlt(eval: &mut evaluate::Evaluate<'_>, args: &Vec<Expr>, scope: &mut HashMap<String, String>) -> String {
    
    let (i1, i2) = Specialforms::check(eval, args, scope);
    return (i1 < i2).to_string();
  }
  
//---------------------------------------------------------
//---------------------------------------------------------

  pub fn check(eval: &mut evaluate::Evaluate<'_>, args: &Vec<Expr>, scope: &mut HashMap<String, String>) -> (i32, i32) {
    
    if args.len() != (2 as usize) {
      println!("Incorrect number of args at define");
      panic!();
    }
    
    let arg1 = eval.evaluate(args[0].clone(), scope);
    let arg2 = eval.evaluate(args[1].clone(), scope);
    
    if !arg1.chars().all(char::is_numeric) &&   !arg2.chars().all(char::is_numeric) {
      println!("Incorrect args at +");
      panic!();
    }
    return (arg1.parse::<i32>().unwrap(), arg2.parse::<i32>().unwrap());
  }
  
  pub fn get(&self, s: &str) -> fn(&mut evaluate::Evaluate<'_>, &Vec<Expr>, &mut HashMap<String, String>) -> String {
    match self.map.get(s) {
            Some(f) => return *f,
            None => panic!(),
        };
  }

  pub fn _type_of<T>(_: T) -> &'static str {
    type_name::<T>()
  }

  pub fn new() -> Self {
    let mut temp: HashMap<_, fn(&mut evaluate::Evaluate<'_>, &Vec<Expr>, &mut HashMap<String, String>) -> String> = HashMap::new();
  
    temp.insert("print",   Specialforms::out);
    temp.insert("println",   Specialforms::outln);
    temp.insert("if",    Specialforms::fnif);
    temp.insert("while", Specialforms::fnwhile);
    temp.insert("do", Specialforms::fndo);
    temp.insert("int", Specialforms::fnint);
    temp.insert("bool", Specialforms::fnbool);
    temp.insert("+", Specialforms::fnadd);
    temp.insert("-", Specialforms::fnsub);
    temp.insert("*", Specialforms::fnmut);
    temp.insert("/", Specialforms::fndiv);
    temp.insert("==", Specialforms::fneq);
    temp.insert("<", Specialforms::fnlt);
    temp.insert(">", Specialforms::fngt);

    return Self {map: temp};
  }
}

