use crate::expr;
use expr::Expr;
use crate::dict::Dict;

pub struct Lexer<'a> {
  dict: Dict<'a>
}

impl Lexer<'_> {

  
  pub fn new() -> Self {

    return Self {dict: Dict::new()};
  }

  pub fn tree(&self, mut soruce: Vec<Expr>) -> Expr {
    return Expr::apply(Expr::word("do"), self.parse(soruce));
  }
  
  pub fn parse(&self, mut soruce: Vec<Expr>) -> Vec<Expr> {

    let mut list: Vec<Expr> = Vec::from([]);
    
    let mut l = 0;
    
      let mut words = soruce.clone();
      
      if soruce.len() > 1 {
        let key: &str = &soruce[0].get_value().to_string();
        let mut i = 0;
        while i<soruce.len() {
          
          if !soruce[i].operator.is_none() {

            //println!("is {:#?}", words[i].get_operator().get_value());
            if soruce[i].get_operator().get_value() == "if" {
            //println!("is if");
            soruce[i] = Expr::apply(Expr::value("if"), vec![
              soruce[i].get_args()[0].clone(),
              self.tree(soruce[i].get_args()[1].get_args())
            ]);
          }
            if soruce[i].get_operator().get_value() == "while" {
            //println!("is if");
            soruce[i] = Expr::apply(Expr::value("while"), vec![
              soruce[i].get_args()[0].clone(),
              self.tree(soruce[i].get_args()[1].get_args())
            ]);
          }
            if soruce[i].get_operator().get_value() == "do" {
           // println!("is do");
            soruce[i] = Expr::apply(Expr::word("do"), 
              self.parse(soruce[i].get_args())
                                  );
          }
          }
          i += 1;
        }
        if self.dict.map.contains_key(&*key) {
          list.push(self.dict.getfn(key)(soruce));
        } else {
          println!("keyword |{}| not found at line {}", soruce[0].get_value(), l+1);
          panic!();
        }
      }    
    //return Expr::apply(Expr::word("do"), list);
    return list;
    //println!("{}", soruce);
  }
  
}