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
      if arr[i] == '(' || arr[i] == ')' {
          //arr.remove(i);
          arr[i] = ' ';
        
      } else if arr[i] == '\n' {
        arr.remove(i);
      }
      i = i+1;
    }
    return arr.into_iter().collect();
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
        //Match words ex: print
        mat = is_word.find_iter(input).map(|x| x.as_str()).collect();
        if mat.len() > 0 {

          let punc = [",", ";", "."];
          let asgmt = ["="];
          let sasgmt = ["+=", "/=", "*=", "-="];
          let math = ["+", "-", "*", "/"];
          let comp = ["==", "!=", "<", "<=", ">", ">="];
          let log = ["&&", "||", "!"];
          let bool = ["true", "false"];
          
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
          if bool.contains(&mat[0]) {
            return Expr::sp_value(mat[0], "bool");
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

  pub fn make_words(line: &str) -> Vec<String> {
    
    let mut in_string = false;
    let mut in_space = false;
    let mut words: Vec<Vec<char>> = Vec::new();
    let mut cw = 0;
    let cha: Vec<char> = line.chars().collect::<Vec<char>>();
    let mut i = 0;
    while i<cha.len() {
      let c = cha[i];
      if in_space && !in_string {
        if c != ' ' {
          in_space = false;
          if c == '"' {
          if !in_string {
            cw = cw + 1;
            words.push(Vec::new());
            words[cw-1].push(c);
          } else {
            words[cw-1].push(c);
            //prevents start new block at end of line
            if !(i == cha.len()-1) {
            cw = cw + 1;
            words.push(Vec::new());
            }
          }
          in_string = !in_string;
          
        } else {
          words.push(Vec::new());
          cw = cw + 1;
          words[cw-1].push(c);
          }
        }
      } else { 
        if c == ' ' && !in_string {
          in_space = true;
        } else if c == '"' {
          if !in_string {
            cw = cw + 1;
            words.push(Vec::new());
            words[cw-1].push(c);
          } else {
            words[cw-1].push(c);
            //prevents start new block at end of line
            if !(i == cha.len()-1) {
            cw = cw + 1;
            words.push(Vec::new());
            }
          }
          in_string = !in_string;
          
        } else {
        if words.len() == 0 {
          cw = cw + 1;
          words.push(Vec::new());
        }
        words[cw-1].push(c);
          
        }
      }
      i = i + 1;
    }

    if words[words.len()-1].len() == 0 {
      words.remove(words.len()-1);
    }
    
    let mut fin: Vec<String> = Vec::new();
    for l in words {
      let temp = l.iter().collect::<String>();
      fin.push(temp);
    }
    return fin.clone();
  }

  
  pub fn make_token(self, mut soruce: String) -> Vec<Vec<Expr>> {
  
    let comment_free = Tokenizer::remove_comments(soruce);
    let par_free = Tokenizer::remove_par(comment_free);
    let lines: Vec<&str> = par_free.split(";").collect();
    let mut lines_expr: Vec<Vec<Expr>> = Vec::new();
    let mut l = 0;
    while l<lines.len() {
      let line = lines[l];
      
      if line != "" {
      let mut line_expr: Vec<Expr> = Vec::new();
      //println!("{}", line);
      let words: Vec<String> = Tokenizer::make_words(line);
      if words.len() > 1 {
        //println!("words: {:#?}", words);
        for w in words {
          line_expr.push(Tokenizer::make_expr(&w));
        }
        
        lines_expr.push(line_expr);
      }
      }
      l = l + 1;
    }

    return lines_expr;
    //println!("{}", soruce);
  }
  
}