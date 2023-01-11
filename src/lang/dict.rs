use crate::expr;
use expr::*;
use std::collections::HashMap;

pub struct Dict {
    pub map: HashMap<String, (fn(Vec<Expr>) -> Expr, usize)>,
}

impl Dict {
    pub fn value_int(part: Vec<Expr>) -> Expr {
        if part.len() == 1 {
            //println!("part: {:#?}", part);
            return part[0].clone();
        } else {
            //return part[part.len()/2].clone();
            let mut op: Vec<Value> = Vec::new();
            let mut index: Vec<usize> = Vec::new();
            let mut i = part.len() - 1;
            let mut mdr_op = false;
            while i > 0 {
                let expr = part[i].clone();
                if !expr.operator.is_none() {
                    if expr.get_operator().get_value() == Value::toString("math") {
                        if [
                            Value::toString("*"),
                            Value::toString("/"),
                            Value::toString("%"),
                        ]
                        .contains(&expr.get_value())
                        {
                            if mdr_op {
                                op.push(part[i].get_value());
                                index.push(i);
                            } else {
                                mdr_op = true;
                                op = Vec::new();
                                index = Vec::new();
                                op.push(part[i].get_value());
                                index.push(i);
                            }
                        } else if !mdr_op {
                            op.push(part[i].get_value());
                            index.push(i);
                        }
                    }
                }
                i = i - 1;
            }

            let l = index[index.len() - 1];
            let mut new_part: Vec<Expr> = Vec::new();
            i = 0;
            while i < part.len() {
                if i == l - 1 || i == l + 1 {
                } else if i == l {
                    let apply = Expr::apply(
                        part[l].clone(),
                        Vec::from([part[l - 1].clone(), part[l + 1].clone()]),
                    );
                    new_part.push(apply.clone());
                } else {
                    new_part.push(part[i].clone());
                }
                i = i + 1;
            }
            return Dict::value_int(new_part);
            //}
        }
    }

    pub fn value_bool(part: Vec<Expr>) -> Expr {
        //println!("part: {:#?}", part.clone());
        if part.len() == 1 {
            if !part[0].operator.is_none() {
                if part[0].get_operator().get_value() == Value::toString("bool")
                    || part[0].get_operator().get_operator().get_value() == Value::toString("comp")
                {
                    return part[0].clone();
                }
            }
            println!("Error at value_bool");
            panic!();
        }

        if part.len() == 2 {
            if !part[0].operator.is_none() {
                if part[0].get_value() == Value::toString("!") {
                    return Expr::apply(part[0].clone(), vec![part[1].clone()]);
                }
            }
            println!("Error at value_bool");
            panic!();
        }

        let mut op: Vec<Value> = Vec::new();
        let mut index: Vec<usize> = Vec::new();
        let mut i = part.len() - 1;
        //println!("|{}|",  part.len());
        let mut ao_op = false;
        while i > 0 {
            //println!("run: {:#?}", part);
            let expr = part[i].clone();
            if !expr.operator.is_none() {
                if expr.get_operator().get_value() == Value::toString("comp")
                    || expr.get_operator().get_value() == Value::toString("log")
                {
                    if [Value::toString("&&"), Value::toString("||")].contains(&expr.get_value()) {
                        if ao_op {
                            op.push(part[i].get_value());
                            index.push(i);
                        } else {
                            ao_op = true;
                            op = Vec::new();
                            index = Vec::new();
                            op.push(part[i].get_value());
                            index.push(i);
                        }
                    } else if !ao_op {
                        op.push(part[i].get_value());
                        index.push(i);
                    }
                }
            }
            i = i - 1;
        }
        let l = index[index.len() / 2];
        if ao_op {
            return Expr::apply(
                part[l].clone(),
                Vec::from([
                    Dict::value_bool(part[..l].to_vec()),
                    Dict::value_bool(part[l + 1..].to_vec()),
                ]),
            );
        } else {
            let mut new_part: Vec<Expr> = Vec::new();
            i = 0;
            let is_not = [Value::toString("!")].contains(&part[l].get_value());
            while i < part.len() {
                if i == l - 1 {
                    if is_not {
                        new_part.push(part[i].clone());
                    }
                } else if i == l + 1 {
                } else if i == l {
                    let apply;
                    if is_not {
                        apply = Expr::apply(part[l].clone(), Vec::from([part[l + 1].clone()]));
                    } else {
                        apply = Expr::apply(
                            part[l].clone(),
                            Vec::from([part[l - 1].clone(), part[l + 1].clone()]),
                        );
                    }
                    new_part.push(apply.clone());
                } else {
                    new_part.push(part[i].clone());
                }
                i = i + 1;
            }
            return Dict::value_bool(new_part);
        }
    }

    pub fn value_string(part: Vec<Expr>) -> Expr {
        if part.len() == 1 {
            //println!("part: {:#?}", part);
            return part[0].clone();
        } else {
            //return part[part.len()/2].clone();
            let mut op: Vec<Value> = Vec::new();
            let mut index: Vec<usize> = Vec::new();
            let mut i = part.len() - 1;
            let mut mdr_op = false;
            while i > 0 {
                let expr = part[i].clone();
                if !expr.operator.is_none() {
                    if expr.get_operator().get_value() == Value::toString("math") {
                        if [Value::toString("*")].contains(&expr.get_value()) {
                            if mdr_op {
                                op.push(part[i].get_value());
                                index.push(i);
                            } else {
                                mdr_op = true;
                                op = Vec::new();
                                index = Vec::new();
                                op.push(part[i].get_value());
                                index.push(i);
                            }
                        } else if !mdr_op {
                            op.push(part[i].get_value());
                            index.push(i);
                        }
                    }
                }
                i = i - 1;
            }

            let l = index[index.len() - 1];
            let mut new_part: Vec<Expr> = Vec::new();
            i = 0;
            while i < part.len() {
                if i == l - 1 || i == l + 1 {
                } else if i == l {
                    let apply = Expr::apply(
                        part[l].clone(),
                        Vec::from([part[l - 1].clone(), part[l + 1].clone()]),
                    );
                    new_part.push(apply.clone());
                } else {
                    new_part.push(part[i].clone());
                }
                i = i + 1;
            }
            return Dict::value_int(new_part);
            //}
        }
    }
    //-----------------------------------------------------------------------

    pub fn fnint(args: Vec<Expr>) -> Expr {
        let mut list: Vec<Expr> = Vec::from([]);
        list.push(args[1].clone());
        if args[2].get_value() == Value::toString("=") {
            list.push(Dict::value_int(args[3..].to_vec()));
        } else {
            println!("Expeted = type at define");
            panic!();
        }
        return Expr::apply(Expr::word(Value::toString("int")), list);
    }

    pub fn fnbool(args: Vec<Expr>) -> Expr {
        let mut list: Vec<Expr> = Vec::from([]);
        list.push(args[1].clone());
        if args[2].get_value() == Value::toString("=") {
            list.push(Dict::value_bool(args[3..].to_vec()));
        } else {
            println!("Expeted = type at define");
            panic!();
        }
        return Expr::apply(Expr::word(Value::toString("int")), list);
    }

    pub fn fnstring(args: Vec<Expr>) -> Expr {
        println!("s: {:?}", args);
        let mut list: Vec<Expr> = Vec::from([]);
        list.push(args[1].clone());
        if args[2].get_value() == Value::toString("=") {
            list.push(Dict::value_string(args[3..].to_vec()));
        } else {
            println!("Expeted = type at define");
            panic!();
        }
        return Expr::apply(Expr::word(Value::toString("string")), list);
    }

    pub fn fnoutput(args: Vec<Expr>) -> Expr {
        return Expr::apply(
            Expr::word(Value::toString("output")),
            Vec::from([args[0].clone()]),
        );
    }

    pub fn fnoutputln(args: Vec<Expr>) -> Expr {
        return Expr::apply(
            Expr::word(Value::toString("outputln")),
            Vec::from([args[0].clone()]),
        );
    }

    pub fn fnif(args: Vec<Expr>) -> Expr {
        let bool = Dict::value_bool(args[0..args.len() - 1].to_vec());
        return Expr::apply(
            Expr::word(Value::toString("if")),
            vec![bool, args[args.len() - 1].clone()],
        );
    }
    pub fn fnwhile(args: Vec<Expr>) -> Expr {
        //println!("{:#?}", args[0..args.len()-1].to_vec());
        let bool = Dict::value_bool(args[0..args.len() - 1].to_vec());
        return Expr::apply(
            Expr::word(Value::toString("while")),
            vec![bool, args[args.len() - 1].clone()],
        );
    }

    //-----------------------------------------------------------------------

    pub fn getfn(&self, s: String) -> fn(Vec<Expr>) -> Expr {
        let (fun, _size) = self.get(s);
        return fun;
    }

    pub fn get(&self, s: String) -> (fn(Vec<Expr>) -> Expr, usize) {
        println!("S: {}", s);
        match self.map.get(&s) {
            Some(f) => return *f,
            None => {
                println!("undefied fn {:?}", s);
                panic!()
            }
        };
    }

    pub fn new() -> Self {
        let mut temp: HashMap<_, (fn(Vec<Expr>) -> Expr, usize)> = HashMap::new();
        temp.insert("int".to_string(), (Dict::fnint, 4));
        temp.insert("bool".to_string(), (Dict::fnbool, 4));
        temp.insert("string".to_string(), (Dict::fnstring, 4));
        temp.insert("output".to_string(), (Dict::fnoutput, 2));
        temp.insert("outputln".to_string(), (Dict::fnoutputln, 2));
        temp.insert("if".to_string(), (Dict::fnif, 2));
        temp.insert("while".to_string(), (Dict::fnwhile, 2));
        temp.insert("value_bool".to_string(), (Dict::value_bool, 0));
        return Self { map: temp };
    }
}
