
use std::collections::HashMap;

//#[path = "specialforms.rs"]
//pub mod specialforms;

//#[path = "parser.rs"]
//pub mod parser;

//#[path = "expr.rs"]
//pub mod expr;

use crate::specialforms;

//use specialforms::expr::Expr;
use crate::expr::Expr;


pub struct Evaluate<'a> {
  pub special_forms: specialforms::Specialforms<'a>
}

impl Evaluate<'_> {
  pub fn evaluate(&mut self, expr: Expr,  mut scope: &mut HashMap<String, String>) -> String {
  
    if expr.get_type() == String::from("value") {
      return expr.value.unwrap();
    } else if expr.type_of == String::from("word") {
      let val = expr.value.unwrap();
      //println!("{:#?}", val);
      if scope.contains_key(&val) {
        return String::from(scope.get(&val).unwrap());
        //return scope[expr.name];
      } else {
        println!("Undefined binding: ${}", val);
        panic!();
      }
    } else if expr.get_type() == String::from("apply") {
      let operator = expr.get_operator();
      let args = expr.get_args();
      let opval = operator.value.unwrap();
      if self.special_forms.map.contains_key(&*opval) {
        //println!("type: {:#?}", opval.clone());
        return self.special_forms.get(&opval)(self, &args, &mut scope);
        //return specialForms[operator.name](expr.args, scope);
      } else {
        println!("|{}| not in special froms", &*opval);
        println!("is {:#?}", expr.get_operator());
        panic!();
      //let op = self.evaluate(operator, scope);
      /*if (typeof op == "function") {
        return op(...args.map(arg => evaluate(arg, scope)));
      } else {
        throw new TypeError("Applying a non-function.");
      }
      */
    }
      
    }
    println!("Error at evaluate type: {}", expr.get_type());
    panic!();
  }

}

