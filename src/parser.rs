  //https://eloquentjavascript.net/12_language.html

//fix number to use i32 insted of String

use regex::Regex;

#[derive(Debug)]
#[derive(Clone)]
pub struct Expr {
  pub type_of: String,
  pub operator: Box<Option<Expr>>,
  pub args: Box<Option<Vec<Expr>>>,
  pub value: Box<Option<String>>
}

impl Expr  {
  pub fn to_string(&self) -> String {
    //return format!("ExprStr {{type: {}, value: {} }}", self.type_of, String::from(self.value));
    //println!("{}", T);
    return format!("ExprStr {{ type: {},\n }}", self.type_of);
  }

  pub fn addto(&mut self, elem: Expr) {
        self.args
            .get_or_insert_with(Vec::new)
            .push(elem);
    }
}


pub struct Parser {
  //pub lexer: lexer::Lexer
}

impl Parser {

  pub fn parse(mut program: &str) -> Expr {
    let (expr, rest) = Parser::parse_expression(program);
    if Parser::skip_space(rest).len() > 0 {
      //println!("Unexpected text after program");
      panic!();
    }
    return expr;
  }

  fn parseApply(mut expr: Expr, mut program: &str) -> (Expr, &str) {
    program = Parser::skip_space(program);
    match program.chars().next() {
        // Get the match slice from string, prints "123"
        Some(..) => (),
        None    => return (expr, program)
    }
    if program.chars().next().unwrap() != '(' {
      //println!("return");
      return (expr, program);
    } 
    program = Parser::skip_space(&program[1..]);
    let mut Fexpr = Expr {
      type_of: Parser::str("apply"),
      operator: Box::new(Some(expr)),
      args: Box::new(Some(Vec::from([]))),
      value: Box::new(None)
    };
    while program.chars().next().unwrap() != ')' {
    //for i in 0..expr.args.unwrap().len() {
      let (arg, rest) = Parser::parse_expression(program);
      Fexpr.addto(arg.clone());
      program = Parser::skip_space(rest);
      //println!("{:#?} - {}", program.chars().next().unwrap(), program);
      if program.chars().next().unwrap() == ',' {
        program = Parser::skip_space(&program[1..]);
      } else if program.chars().next().unwrap() != ')' {
        //println!("SyntaxError: Expected , or ) at {}", program);
        panic!();
      }
      //println!("{:#?} :: {:#?}", Fexpr.args.clone().unwrap(), arg);
    }
    //println!("{:#?}", );
    //println!("{:#?}, {}", expr.to_string(), program);
    return Parser::parseApply(Fexpr, &program[1..]);
  }
  
  fn parse_expression(mut program: &str) -> (Expr, &str) {
    program = Parser::skip_space(program);
    let mut mat: Vec<&str>;//, expr;
    //let mut expr: Expr =  Expr {type_of: String::from(""), value: String::from("")};
    //r#""#
    //^"([^"]*)"
    let is_string = Regex::new(r#"^"([^"]*)""#).unwrap();
    let is_number = Regex::new(r#"^\d+\b"#).unwrap();
    let is_word = Regex::new(r#"^[^\s(),#"]+"#).unwrap();

    //Match Strings ex: "text"
     mat = is_string.find_iter(program).map(|x| x.as_str()).collect();
    if mat.len() > 0 {
      //println!("{:#?}", mat);
      let expr = Expr {
        type_of: String::from("value"), 
        value: Box::new(Some(String::from(mat[0]))),
        operator: Box::new(None), 
        args: Box::new(None)
      };
      return Parser::parseApply(expr, &program[mat[0].len()..]);
    } else {
      //Match Numbers ex: 10
      mat = is_number.find_iter(program).map(|x| x.as_str()).collect();
      if mat.len() > 0 {
        let expr = Expr {
          type_of: String::from("value"), 
          value: Box::new(Some(String::from(mat[0]))), 
          operator: Box::new(None), 
          args: Box::new(None) 
        };
        return Parser::parseApply(expr, &program[mat[0].len()..]);
      } else {
        //Match Numbers ex: 10
        mat = is_word.find_iter(program).map(|x| x.as_str()).collect();
        if mat.len() > 0 {
          let expr = Expr {
            type_of: String::from("word"), 
            value: Box::new(Some(String::from(mat[0]))), 
            operator: Box::new(None), 
            args: Box::new(None)
        };
          return Parser::parseApply(expr, &program[mat[0].len()..]);
        } else {
          println!("Unexpected syntax: {}", program);
          panic!();
      //  return parseApply(expr, program.slice(match[0].length));

        }
      }
    }
    
  //println!("{}, {:#?}, {}", mat.len(), mat, expr.to_string());
  
  }

  fn skip_space(string: &str) -> &str {
    let first = Regex::new(r"\S").unwrap();
    if !first.is_match(string) {
      return "";
    }

    match first.find(string) {
        // Get the match slice from string, prints "123"
        Some(x) => return &string[x.start()..],
        None    => unreachable!()
    }
  }

  fn str(s: &str) -> String {
    return String::from(s);
  }
}

//dyn std::any::Any
//impl std::any::Any