use std::io;
use std::io::Write; // <--- bring flush() into scope

use std::collections::HashMap;

//use crate::parser;
use crate::evaluate;
use crate::expr;

//#[path = "parser.rs"]
//pub mod parser;

//#[path = "expr.rs"]
//pub mod expr;

//#[path = "evaluate.rs"]
//pub mod evaluate;
//use evaluate::*;
use expr::*;

pub struct Specialforms {
    pub map: HashMap<
        String,
        fn(&mut evaluate::Evaluate, &Vec<Expr>, &mut HashMap<String, Value>) -> Value,
    >,
}

impl Specialforms {
    //---------------------------------------------------------
    //---------------------------------------------------------
    pub fn outputln(
        eval: &mut evaluate::Evaluate,
        args: &Vec<Expr>,
        scope: &mut HashMap<String, Value>,
    ) -> Value {
        if args.len() != (1 as usize) {
            println!("Incorrect number of args at out");
            panic!();
        }
        //let elm_expr = &args.as_ref().as_ref().unwrap()[0];
        //let val = elm_expr.value.as_ref().as_ref().unwrap();
        let raw = eval.evaluate(args[0].clone(), scope).to_string();
        if raw.contains('"') {
            let mes = &raw[1..raw.len() - 1];
            println!("{}", mes);
        } else {
            println!("{}", raw);
        }

        return Value::Bool(true);
    }

    pub fn output(
        eval: &mut evaluate::Evaluate,
        args: &Vec<Expr>,
        scope: &mut HashMap<String, Value>,
    ) -> Value {
        if args.len() != (1 as usize) {
            println!("Incorrect number of args at out");
            panic!();
        }
        //let elm_expr = &args.as_ref().as_ref().unwrap()[0];
        //let val = elm_expr.value.as_ref().as_ref().unwrap();
        let raw = eval.evaluate(args[0].clone(), scope).to_string();
        if raw.contains('"') {
            let mes = &raw[1..raw.len() - 1];
            print!("{}", mes);
        } else {
            print!("{}", raw);
        }
        io::stdout().flush().unwrap();
        return Value::Bool(true);
    }

    pub fn fnif(
        eval: &mut evaluate::Evaluate,
        args: &Vec<Expr>,
        scope: &mut HashMap<String, Value>,
    ) -> Value {
        if args.len() != (2 as usize) {
            println!("Incorrect number of args at if");
            panic!();
        } else if eval.evaluate(args[0].clone(), scope) == Value::Bool(true) {
            return eval.evaluate(args[1].clone(), scope);
        } /* else {
            return  eval.evaluate(args[2].clone(), scope);
          } */
        return Value::Bool(true);
    }

    pub fn fnwhile(
        eval: &mut evaluate::Evaluate,
        args: &Vec<Expr>,
        scope: &mut HashMap<String, Value>,
    ) -> Value {
        if args.len() != (2 as usize) {
            println!("Incorrect number of args at while");
            panic!();
        }
        while eval.evaluate(args[0].clone(), scope) != Value::Bool(false) {
            eval.evaluate(args[1].clone(), scope);
        }

        return Value::Bool(true);
    }

    pub fn fndo(
        eval: &mut evaluate::Evaluate,
        args: &Vec<Expr>,
        scope: &mut HashMap<String, Value>,
    ) -> Value {
        let mut value = Value::Bool(false);
        for arg in args {
            //println!("arg: {:#?}", arg);
            value = eval.evaluate(arg.clone(), scope);
        }
        return value;
    }

    pub fn fnint(
        eval: &mut evaluate::Evaluate,
        args: &Vec<Expr>,
        scope: &mut HashMap<String, Value>,
    ) -> Value {
        if args.len() != (2 as usize) || args[0].type_of != Type::Word {
            println!("Incorrect number of args at define");
            panic!();
        }
        let value = eval.evaluate(args[1].clone(), scope);
        let name = args[0].get_value();
        scope.insert(name.to_string(), value.clone());
        return value;
    }

    pub fn fnbool(
        eval: &mut evaluate::Evaluate,
        args: &Vec<Expr>,
        scope: &mut HashMap<String, Value>,
    ) -> Value {
        if args.len() != (2 as usize) || args[0].type_of != Type::Word {
            println!("Incorrect number of args at define");
            panic!();
        }
        let value = eval.evaluate(args[1].clone(), scope);
        let name = args[0].get_value();
        scope.insert(name.to_string(), value.clone());
        return value;
    }

    pub fn fnstring(
        eval: &mut evaluate::Evaluate,
        args: &Vec<Expr>,
        scope: &mut HashMap<String, Value>,
    ) -> Value {
        if args.len() != (2 as usize) || args[0].type_of != Type::Word {
            println!("Incorrect number of args at string");
            panic!();
        }
        let value = eval.evaluate(args[1].clone(), scope);
        match &value {
            Value::String(_x) => (),
            _ => {
                println!("Incorrect type of value at string");
                panic!();
            }
        }

        let name = args[0].get_value();
        scope.insert(name.to_string(), value.clone());
        return value;
    }

    pub fn fnadd(
        eval: &mut evaluate::Evaluate,
        args: &Vec<Expr>,
        scope: &mut HashMap<String, Value>,
    ) -> Value {
        if args.len() != (2 as usize) {
            println!("Incorrect number of args at math op");
            panic!();
        }

        let arg1 = eval.evaluate(args[0].clone(), scope);
        let arg2 = eval.evaluate(args[1].clone(), scope);

        match (arg1, arg2) {
            (Value::String(s1), Value::String(s2)) => {
                return Value::String(s1 + &s2);
            }
            (Value::Int(i1), Value::Int(i2)) => {
                let (i1, i2) = Specialforms::check(eval, args, scope);
                return Value::Int(i1 + i2);
            }
            _ => {
                println!("Unexpected types at +");
                panic!();
            }
        }
    }

    pub fn fnsub(
        eval: &mut evaluate::Evaluate,
        args: &Vec<Expr>,
        scope: &mut HashMap<String, Value>,
    ) -> Value {
        let (i1, i2) = Specialforms::check(eval, args, scope);
        return Value::Int(i1 - i2);
    }
    pub fn fnmut(
        eval: &mut evaluate::Evaluate,
        args: &Vec<Expr>,
        scope: &mut HashMap<String, Value>,
    ) -> Value {
        let (i1, i2) = Specialforms::check(eval, args, scope);
        return Value::Int(i1 * i2);
    }
    pub fn fndiv(
        eval: &mut evaluate::Evaluate,
        args: &Vec<Expr>,
        scope: &mut HashMap<String, Value>,
    ) -> Value {
        let (i1, i2) = Specialforms::check(eval, args, scope);
        return Value::Int(i1 / i2);
    }

    pub fn fnmod(
        eval: &mut evaluate::Evaluate,
        args: &Vec<Expr>,
        scope: &mut HashMap<String, Value>,
    ) -> Value {
        let (i1, i2) = Specialforms::check(eval, args, scope);
        return Value::Int(i1 % i2);
    }

    pub fn fneq(
        eval: &mut evaluate::Evaluate,
        args: &Vec<Expr>,
        scope: &mut HashMap<String, Value>,
    ) -> Value {
        let (i1, i2) = Specialforms::check(eval, args, scope);
        return Value::Bool(i1 == i2);
    }

    pub fn fnneq(
        eval: &mut evaluate::Evaluate,
        args: &Vec<Expr>,
        scope: &mut HashMap<String, Value>,
    ) -> Value {
        let (i1, i2) = Specialforms::check(eval, args, scope);
        return Value::Bool(i1 != i2);
    }

    pub fn fngt(
        eval: &mut evaluate::Evaluate,
        args: &Vec<Expr>,
        scope: &mut HashMap<String, Value>,
    ) -> Value {
        let (i1, i2) = Specialforms::check(eval, args, scope);
        return Value::Bool(i1 > i2);
    }

    pub fn fnlt(
        eval: &mut evaluate::Evaluate,
        args: &Vec<Expr>,
        scope: &mut HashMap<String, Value>,
    ) -> Value {
        let (i1, i2) = Specialforms::check(eval, args, scope);
        return Value::Bool(i1 < i2);
    }

    pub fn fnand(
        eval: &mut evaluate::Evaluate,
        args: &Vec<Expr>,
        scope: &mut HashMap<String, Value>,
    ) -> Value {
        if args.len() != (2 as usize) {
            println!("Incorrect number of args at define");
            panic!();
        }

        let arg1 = eval.evaluate(args[0].clone(), scope);
        let arg2 = eval.evaluate(args[1].clone(), scope);
        if arg1 == Value::Bool(true) && arg2 == Value::Bool(true) {
            return Value::Bool(true);
        } else {
            return Value::Bool(false);
        }
    }

    pub fn fnor(
        eval: &mut evaluate::Evaluate,
        args: &Vec<Expr>,
        scope: &mut HashMap<String, Value>,
    ) -> Value {
        if args.len() != (2 as usize) {
            println!("Incorrect number of args at define");
            panic!();
        }

        let arg1 = eval.evaluate(args[0].clone(), scope);
        let arg2 = eval.evaluate(args[1].clone(), scope);
        if arg1 == Value::Bool(true) || arg2 == Value::Bool(true) {
            return Value::Bool(true);
        } else {
            return Value::Bool(false);
        }
    }

    pub fn fnnot(
        eval: &mut evaluate::Evaluate,
        args: &Vec<Expr>,
        scope: &mut HashMap<String, Value>,
    ) -> Value {
        if args.len() != (1 as usize) {
            println!("Incorrect number of args at define");
            panic!();
        }

        let arg1 = eval.evaluate(args[0].clone(), scope);
        if !(arg1 == Value::Bool(true)) {
            return Value::Bool(true);
        } else {
            return Value::Bool(false);
        }
    }

    //---------------------------------------------------------
    //---------------------------------------------------------

    pub fn check(
        eval: &mut evaluate::Evaluate,
        args: &Vec<Expr>,
        scope: &mut HashMap<String, Value>,
    ) -> (i32, i32) {
        if args.len() != (2 as usize) {
            println!("Incorrect number of args at math op");
            panic!();
        }

        let arg1 = eval.evaluate(args[0].clone(), scope).to_string();
        let arg2 = eval.evaluate(args[1].clone(), scope).to_string();

        if !arg1.chars().all(char::is_numeric) && !arg2.chars().all(char::is_numeric) {
            println!("Incorrect args at +");
            panic!();
        }
        return (arg1.parse::<i32>().unwrap(), arg2.parse::<i32>().unwrap());
    }

    pub fn get(
        &self,
        s: &str,
    ) -> fn(&mut evaluate::Evaluate, &Vec<Expr>, &mut HashMap<String, Value>) -> Value {
        match self.map.get(s) {
            Some(f) => return *f,
            None => panic!(),
        };
    }

    pub fn new() -> Self {
        let mut temp: HashMap<
            _,
            fn(&mut evaluate::Evaluate, &Vec<Expr>, &mut HashMap<String, Value>) -> Value,
        > = HashMap::new();

        temp.insert("output".to_string(), Specialforms::output);
        temp.insert("outputln".to_string(), Specialforms::outputln);
        temp.insert("if".to_string(), Specialforms::fnif);
        temp.insert("while".to_string(), Specialforms::fnwhile);
        temp.insert("do".to_string(), Specialforms::fndo);
        temp.insert("int".to_string(), Specialforms::fnint);
        temp.insert("bool".to_string(), Specialforms::fnbool);
        temp.insert("string".to_string(), Specialforms::fnstring);
        temp.insert("+".to_string(), Specialforms::fnadd);
        temp.insert("-".to_string(), Specialforms::fnsub);
        temp.insert("*".to_string(), Specialforms::fnmut);
        temp.insert("/".to_string(), Specialforms::fndiv);
        temp.insert("%".to_string(), Specialforms::fnmod);
        temp.insert("==".to_string(), Specialforms::fneq);
        temp.insert("!=".to_string(), Specialforms::fnneq);
        temp.insert("<".to_string(), Specialforms::fnlt);
        temp.insert(">".to_string(), Specialforms::fngt);
        temp.insert("&&".to_string(), Specialforms::fnand);
        temp.insert("||".to_string(), Specialforms::fnor);
        temp.insert("!".to_string(), Specialforms::fnnot);

        return Self { map: temp };
    }
}
