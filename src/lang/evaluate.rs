
use std::collections::HashMap;
//use std::ops::Deref;

use crate::specialforms;

use crate::expr::*;
use crate::scope::*;

pub struct Evaluate {
  pub special_forms: specialforms::Specialforms
}

impl Evaluate {
  pub fn evaluate(&mut self, expr: Expr,  mut scope: &mut Scope) -> Value { //-> String
  
    if expr.get_type() == Type::Value {
      return expr.value;
    } else if expr.type_of == Type::Word {
      let val = expr.value.to_string();
      //println!("{:#?}", val);
      match scope.findget(&val) {
        Some(v) => return v,
        None => {
           println!("Undefined binding: ${:?}", val);
        panic!();
        }
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
        let mut args: Vec<Value> = vec!();
        for e in expr.get_args() {
          args.push(e.get_value());
        }
        //println!("{:?}", args);
        match scope.isfn(&opval) {
        Some(e) => {
          let (stat, args) = e;
          let prams = expr.get_args();
          if args.len() != prams.len() {
            println!("Improuper ammount of args at {} call", &opval);
            panic!();
          }
          let mut nscope = Scope::new();
          for i in 0..prams.len() {
            let parm = prams[i].get_value();
            let (name, _val) = args[i].clone();
            nscope.insert(name, parm);
            //println!("|{:?}|", parm);
          }
          return self.evaluate(stat, &mut nscope);
        }
        None => {
          println!("|{}| not in special froms", &*opval);
          println!("is {:#?}", expr.get_operator());
          panic!();
                }
      }
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