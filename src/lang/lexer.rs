use crate::expr;
use expr::Expr;
use crate::dict::Dict;

pub struct Lexer<'a> {
  dict: Dict<'a>,
  source: Vec<Expr>,
  i: usize,
  key: String,
  match_c: char,
  is_control: bool,
  current: Vec<Expr>,
  list: Vec<Expr>,
  cond: Expr,
  ce: Expr,
  pc: i8,
  bc: i8
}

impl Lexer<'_> {

  
  pub fn new(mut soruce: Vec<Expr>) -> Self {
    return Self {
      dict: Dict::new(),
      source: soruce,
      i: 0,
      key: String::from(""),
      match_c: '(',
      is_control: false,
      current: vec!(),
      list: vec!(),
      cond: Expr::empty(),
      ce: Expr::empty(),
      pc: 0,
      bc: 0,
    };
  }

  pub fn tree(&mut self) -> Expr {
    //println!("=========tree===========s");
    return Expr::apply(Expr::word("do"), self.parse());
  }
  
  pub fn parse(&mut self) -> Vec<Expr> {
    //println!("==========parse==========s");
    while self.i<self.source.len() {
      if !self.ce.value.is_none() {
        //println!("----------{:#?}----{}-----", self.ce.get_value(), self.bc);
      }
      
      self.ce = self.source[self.i].clone();
      self.count();
      self.next();
      if !self.ce.value.is_none() {
        //println!("----------{:#?}----{}-----", self.ce.get_value(), self.bc);
      }
      self.i += 1;
      
    }
    //println!("==========parse==========e");
    //println!("tree: {:#?}", self.list.clone());
    return self.list.clone();
  }

  pub fn count(&mut self) {
    //count ) and }
    if self.ce.get_value() == "(" {
      self.pc += 1;
    }
    if self.ce.get_value() == ")" {
      self.pc -= 1;
    }
    if self.ce.get_value() == "{" {
      self.bc += 1;
    }
    if self.ce.get_value() == "}" {
      self.bc -= 1;
    }
  }
  
  pub fn next(&mut self) {
    if self.match_c == ')' || self.match_c == '}' {
      self.end();
    } else {
      self.start();
    }
  }

  pub fn start(&mut self) {
    //check of op == ctrl
    if !self.ce.operator.is_none() && self.ce.get_operator().get_value() == "key" {
      let mut temp: Vec<Expr> = vec!();
      while self.ce.operator.is_none() || !(self.ce.get_operator().get_value() == "end") {
        self.ce = self.source[self.i].clone();
        temp.push(self.ce.clone());
        self.i += 1;
      }
      self.i -= 1;
      temp.pop();
      self.list.push(self.dict.getfn(temp[0].get_value())(temp));
      
      
    }
    if !self.ce.operator.is_none() && self.ce.get_operator().get_value() == "ctrl" {
      self.key = String::from(self.ce.get_value());
      self.is_control = true;
    } else if self.value_match() {
      if self.is_control {
        if self.match_c == '{' {
          self.match_c = '}';
      } else {
          self.match_c = ')';
      }
    } 
    else {
      self.key = String::from(self.source[self.i-1].get_value()); //check if i > 0
      self.match_c = ')';
      }
    }
  }

  pub fn end(&mut self) {
    if self.value_match() {
      if self.is_control {
        if self.match_c == ')' {
          self.match_c = '{';
          self.cond = self.dict.getfn("value_bool")(self.current.clone());
          self.current = vec!();
        } else if self.match_c == '}' {
          if self.bc == 0 {
          //println!("{:#?}", self.current);
          /////////////////
          let mut lex = Lexer::new(self.current.clone());
          let mut temp = vec![self.cond.clone(), lex.tree()];
          /* for i in 0..self.current.len() {
            temp.push(self.current[i].clone());
          } */
          self.list.push(Expr::apply(Expr::value(&self.key), temp));
          self.current = vec!();
          self.is_control = false;
          self.match_c = '(';
        } else {
            //println!("#########{:#?}########", self.bc == 0);
          self.current.push(self.ce.clone());
        }
        }
      } else {
        //println!("-----####--{:#?}", self.current.clone());
        self.list.push(self.dict.getfn(&self.key)(self.current.clone()));
        self.current = vec!();
        self.match_c = '(';
      }
    } else {
      self.current.push(self.ce.clone());
    }
  }

  pub fn value_match (&mut self) -> bool{
    self.ce.get_value() == &String::from(self.match_c)
  }
  
}