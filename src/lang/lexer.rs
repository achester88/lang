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

  pub fn remove_comments(input: String) -> String {
    let mut arr: Vec<char> = input.chars().collect();
    let mut i=0;
    while i < arr.len() {
      if arr[i] == '/' && arr[i+1] == '/' {
        while arr[i] !='\n' {
          arr.remove(i);
        }
      }
      i = i+1;
    }
    return arr.into_iter().collect();
  }


  
  pub fn remove_par(input: String) -> String {
    let mut arr: Vec<char> = input.chars().collect();
    let mut i=0;
    while i < arr.len() {
      if arr[i] == '(' || arr[i] == ')' || arr[i] == '\n' {
          //arr.remove(i);
          arr[i] = ' ';
        
      }
      i = i+1;
    }
    return arr.into_iter().collect();
  }


  pub fn remove_empty(mut input: Vec<&str>) -> Vec<&str> {
    let mut i=0;
    while i < input.len() {
      if input[i] == "" {
          input.remove(i);
        
      }
      i = i+1;
    }
    return input;
  }

  pub fn tree(&self, mut soruce: Vec<Vec<Expr>>) -> Expr {
    return Expr::apply(Expr::word("do"), self.parse(soruce));
  }
  
  pub fn parse(&self, mut soruce: Vec<Vec<Expr>>) -> Vec<Expr> {

    let mut list: Vec<Expr> = Vec::from([]);
    
    let mut l = 0;
    while l<soruce.len() {
      let mut words = soruce[l].clone();
      
      if words.len() > 1 {
        let key: &str = &words[0].get_value().to_string();
        let mut i = 0;
        while i<words.len() {
          
          if !words[i].operator.is_none() {

            //println!("is {:#?}", words[i].get_operator().get_value());
            if words[i].get_operator().get_value() == "if" {
            //println!("is if");
            words[i] = Expr::apply(Expr::value("if"), vec![
              words[i].get_args()[0].clone(),
              self.tree(vec![words[i].get_args()[1].get_args()])
            ]);
          }
            if words[i].get_operator().get_value() == "while" {
            //println!("is if");
            words[i] = Expr::apply(Expr::value("while"), vec![
              words[i].get_args()[0].clone(),
              self.tree(vec![words[i].get_args()[1].get_args()])
            ]);
          }
            if words[i].get_operator().get_value() == "do" {
           // println!("is do");
            words[i] = Expr::apply(Expr::word("do"), 
              self.parse(vec![words[i].get_args()])
                                  );
          }
          }
          i += 1;
        }
        if self.dict.map.contains_key(&*key) {
          list.push(self.dict.getfn(key)(words));
        } else {
          println!("keyword |{}| not found at line {}", words[0].get_value(), l+1);
          panic!();
        }
      }
      l = l + 1;
    }
    //return Expr::apply(Expr::word("do"), list);
    return list;
    //println!("{}", soruce);
  }
  
}