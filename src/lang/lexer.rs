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

        
  pub fn parse(self, mut soruce: Vec<Vec<Expr>>) -> Expr {

    let mut list: Vec<Expr> = Vec::from([]);
    
    let mut l = 0;
    while l<soruce.len() {
      let words = soruce[l].clone();
      
      if words.len() > 1 {
        let key: &str = words[0].get_value();
        if self.dict.map.contains_key(&*key) {
          list.push(self.dict.getfn(key)(words));
        } else {
          println!("keyword {} not found at line {}", words[0].get_value(), l+1);
        }
      }
      l = l + 1;
    }
    return Expr::apply(Expr::word("do"), list);
    //println!("{}", soruce);
  }
  
}