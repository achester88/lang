use std::collections::HashMap;

#[path = "specialforms.rs"]
pub mod specialforms;

#[path = "parser.rs"]
pub mod parser;

#[path = "expr.rs"]
pub mod expr;

use specialforms::expr::Expr;

pub struct Specialforms<'a> {
  special_forms: specialforms::Specialforms<'a>
}

impl Specialforms<'_> {
  pub fn evaluate(&mut self, expr: Expr, mut scope: HashMap<String, String>) -> String {
  
    if expr.type_of == String::from("value") {
      return expr.value.unwrap();
    } else if expr.type_of == String::from("word") {
      let val = expr.value.unwrap();
      println!("{:#?}", val);
      if scope.contains_key(&val) {
        return String::from(scope.get(&val).unwrap());
        //return scope[expr.name];
      } else {
        println!("Undefined binding: ${}", val);
        panic!();
      }
    } else if expr.type_of == String::from("apply") {
      let operator = expr.operator.unwrap();
      let args = expr.args;
      let opval = operator.value.unwrap();
      if operator.type_of == String::from("word") && self.special_forms.map.contains_key(&*opval) {
        return self.special_forms.get(&opval)(&args, &scope);
        //return specialForms[operator.name](expr.args, scope);
      } else {
        let op = self.evaluate(expr, scope);
      }
    }
    println!("Error at evaluate");
    panic!();
  }
}