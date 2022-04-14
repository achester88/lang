use regex::Regex;
use crate::expr;
use expr::Expr;

pub struct Tokenizer {
}

impl Tokenizer {

  
  pub fn new() -> Self {

    return Self {};
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

  pub fn make_expr(input: &str) -> Expr {
    let mut mat: Vec<&str>;//, expr;
    //let mut expr: Expr =  Expr {type_of: String::from(""), value: String::from("")};
    //r#""#
    //^"([^"]*)"
    let is_string = Regex::new(r#"^"([^"]*)""#).unwrap();
    let is_number = Regex::new(r#"^\d+\b"#).unwrap();
    let is_word = Regex::new(r#"^[^\s(),#"]+"#).unwrap();

    //Match Strings ex: "text"
     mat = is_string.find_iter(input).map(|x| x.as_str()).collect();
    if mat.len() > 0 {
      //println!("{:#?}", mat);
      return Expr::value(mat[0]);
    } else {
      //Match Numbers ex: 10
      mat = is_number.find_iter(input).map(|x| x.as_str()).collect();
      if mat.len() > 0 {
        return Expr::value(mat[0]);
      } else {
        //Match Numbers ex: 10
        mat = is_word.find_iter(input).map(|x| x.as_str()).collect();
        if mat.len() > 0 {

          let punc = [",", ";", "."];
          let asgmt = ["="];
          let sasgmt = ["+=", "/=", "*=", "-="];
          let math = ["+", "-", "*", "/"];
          let comp = ["==", "!=", "<", "<=", ">", ">="];
          let log = ["&&", "||", "!"];
          
          if punc.contains(&mat[0]) {
            return Expr::sp_word(mat[0], "punc");
          }
          if asgmt.contains(&mat[0]) {
            return Expr::sp_word(mat[0], "asgmt");
          }
          if sasgmt.contains(&mat[0]) {
            return Expr::sp_word(mat[0], "sasgmt");
          }
          if comp.contains(&mat[0]) {
            return Expr::sp_word(mat[0], "comp");
          }
          if log.contains(&mat[0]) {
            return Expr::sp_word(mat[0], "log");
          }
          if math.contains(&mat[0]) {
            return Expr::sp_word(mat[0], "math");
          }
          return Expr::word(mat[0]);
        } else {
          println!("Unexpected syntax: |{}|", input);
          panic!();
      //  return parse_apply(expr, program.slice(match[0].length));

        }
      }
    }
  }
  
  pub fn make_token(self, mut soruce: String) -> Vec<Vec<Expr>> {

    
    let comment_free = Tokenizer::remove_comments(soruce);
    let par_free = Tokenizer::remove_par(comment_free);
    let lines: Vec<&str> = par_free.split(";").collect();
    
    let mut lines_expr: Vec<Vec<Expr>> = Vec::new();
    let mut l = 0;
    
    while l<lines.len() {
      let line = lines[l];
      let mut line_expr: Vec<Expr> = Vec::new();
      //println!("{}", line);
      let words: Vec<&str> = Tokenizer::remove_empty(line.split(" ").collect());
      if words.len() > 1 {
        for w in words {
          line_expr.push(Tokenizer::make_expr(w));
        }
        
        lines_expr.push(line_expr);
      }
      l = l + 1;
    }

    return lines_expr;
    //println!("{}", soruce);
  }
  
}