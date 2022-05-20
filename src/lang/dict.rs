use std::collections::HashMap;
use crate::expr;
use crate::lexer;
use expr::*;

pub struct Dict<'a> {
  pub map: HashMap<&'a str, (fn(Vec<Expr>) -> Expr, usize)>
}

impl Dict<'_> {

  pub fn value_int(part: Vec<Expr>) -> Expr {
    if part.len() == 1 {
      //println!("part: {:#?}", part);
      return part[0].clone();
    } else {
      //return part[part.len()/2].clone();
      let mut op: Vec<&str> = Vec::new();
      let mut index: Vec<usize> = Vec::new();
      let mut i = part.len()-1;
      let mut mdr_op = false;
      while i>0 {
        
        let expr = part[i].clone();
      if !expr.operator.is_none()  {
        if expr.get_operator().get_value() == "math" {
          if ["*", "/", "%"].contains(&expr.get_value()) {
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
      
       let l = index[index.len()-1];
        let mut new_part: Vec<Expr> = Vec::new();
        i = 0;
        while i<part.len() {
          if i==l-1 || i==l+1 {
          } else if i==l {
            let apply = Expr::apply(
                part[l].clone(),
                Vec::from([
                  part[l-1].clone(),
                   part[l+1].clone()
                ])
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
    if part.len() == 1 {
      if !part[0].operator.is_none() {
        
      if part[0].get_operator().get_value() == "bool" || part[0].get_operator().get_operator().get_value() == "comp" {
        return part[0].clone();
        }
      }
      println!("Error at value_bool");
      panic!();
    }

  if part.len() == 2 {
    if !part[0].operator.is_none() {
      if part[0].get_value() == "!" {
        return Expr::apply(
           part[0].clone(),
          vec![part[1].clone()]
        );
      }
    }
    println!("Error at value_bool");
    panic!();
  }

    let mut op: Vec<&str> = Vec::new();
    let mut index: Vec<usize> = Vec::new();
    let mut i = part.len()-1;
    let mut ao_op = false;
    while i>0 {
      //println!("run: {:#?}", part);
        let expr = part[i].clone();
      if !expr.operator.is_none()  {
        if expr.get_operator().get_value() == "comp" || expr.get_operator().get_value() == "log" {
          if ["&&", "||"].contains(&expr.get_value()) {
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
      let l = index[index.len()/2];
      if ao_op {
        return Expr::apply(
                part[l].clone(),
                Vec::from([
                   Dict::value_bool(part[..l].to_vec()),
                  Dict::value_bool(part[l+1..].to_vec())
                ])
              );
      } else {
        let mut new_part: Vec<Expr> = Vec::new();
        i = 0;
        let is_not = ["!"].contains(&part[l].get_value());
        while i<part.len() {
          if i ==l-1 {
            if is_not {
              new_part.push(part[i].clone());
            }
          }
          else if i==l+1 {
          } else if i==l {
            let apply;
            if is_not {
            apply = Expr::apply(
                part[l].clone(),
                Vec::from([
                   part[l+1].clone()
                ])
              );
            } else {
            apply = Expr::apply(
                part[l].clone(),
                Vec::from([
                  part[l-1].clone(),
                   part[l+1].clone()
                ])
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
//-----------------------------------------------------------------------
  
  pub fn fnint(args: Vec<Expr>) -> Expr {
    let mut list: Vec<Expr> = Vec::from([]);
    list.push(args[1].clone());
    if args[2].get_value() == "=" {
    list.push(Dict::value_int(args[3..].to_vec()));
    } else {
      println!("Expeted = type at define");
      panic!();
    }
    return Expr::apply(Expr::word("int"), list);
  }

  pub fn fnbool(args: Vec<Expr>) -> Expr {
    let mut list: Vec<Expr> = Vec::from([]);
    list.push(args[1].clone());
    if args[2].get_value() == "=" {
    list.push(Dict::value_bool(args[3..].to_vec()));
    } else {
      println!("Expeted = type at define");
      panic!();
    }
    return Expr::apply(Expr::word("int"), list);
  }

  pub fn fnout(args: Vec<Expr>) -> Expr {
    return Expr::apply(Expr::word("print"), Vec::from([args[1].clone()]));
  }

  pub fn fnoutln(args: Vec<Expr>) -> Expr {
    return Expr::apply(Expr::word("println"), Vec::from([args[1].clone()]));
  }

  pub fn fnif(args: Vec<Expr>) -> Expr {
    let bool = Dict::value_bool(args[1..args.len()-1].to_vec());
    return Expr::apply(
      Expr::word("if"),
      vec![ bool, args[args.len()-1].clone() ]
    );
  }
  pub fn fnwhile(args: Vec<Expr>) -> Expr {
    println!("{:#?}", args[1..args.len()-1].to_vec());
    let bool = Dict::value_bool(args[1..args.len()-1].to_vec());
    return Expr::apply(
      Expr::word("while"),
      vec![ bool, args[args.len()-1].clone() ]
    );
  }

//-----------------------------------------------------------------------

  
  pub fn getfn(&self, s: &str) -> fn(Vec<Expr>) -> Expr {
    let (fun, _size) = self.get(s);
    return fun;
  }
  
  pub fn get(&self, s: &str) -> (fn(Vec<Expr>) -> Expr, usize) {
    match self.map.get(s) {
            Some(f) => return *f,
            None => panic!(),
        };
  }
  
  pub fn new() -> Self {
    let mut temp: HashMap<_, (fn(Vec<Expr>) -> Expr, usize)> = HashMap::new();
    temp.insert("int", (Dict::fnint, 4));
    temp.insert("bool", (Dict::fnbool, 4));
    temp.insert("print", (Dict::fnout, 2));
    temp.insert("println", (Dict::fnoutln, 2));
    temp.insert("if", (Dict::fnif, 2));
    temp.insert("while", (Dict::fnwhile, 2));
    return Self {map: temp};
  }
  
}