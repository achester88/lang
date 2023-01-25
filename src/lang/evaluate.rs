
use std::collections::HashMap;
//use std::ops::Deref;

use crate::specialforms;

use crate::expr::*;


pub struct Evaluate {
  pub special_forms: specialforms::Specialforms
}

impl Evaluate {
  pub fn evaluate(&mut self, expr: Expr,  mut scope: &mut HashMap<String, Value>) -> Value { //-> String
  
    if expr.get_type() == Type::Value {
      return expr.value;
    } else if expr.type_of == Type::Word {
      let val = expr.value.to_string();
      //println!("{:#?}", val);
      if scope.contains_key(&val) {
        return scope.get(&val).unwrap().to_owned();
        //return scope[expr.name];
      } else {
        println!("Undefined binding: ${:?}", val);
        panic!();
      }
    } else if expr.get_type() == Type::Apply {
      let operator = expr.get_operator();
      let args = expr.get_args();
      let opval = operator.value.to_string();
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
    println!("Error at evaluate type: {:?}", expr.get_type());
    panic!();
  }

}