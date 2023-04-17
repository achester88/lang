use crate::expr::*;
use crate::errorhandler::ErrorHandler;
pub struct Preprocesser<'a> {
  tokens: Vec<Expr>,
  pos: &'a Vec<(u32, u32)>,
  i: usize,
  bc: i32,
  pc: i32,
  error_handler: ErrorHandler
}

impl<'a> Preprocesser<'a> {
  pub fn new(tokens: Vec<Expr>, pos: &'a Vec<(u32, u32)>, con: Vec<String>) -> Self {
    return Self {
      tokens: tokens,
      pos: pos,
      i: 0,
      bc: 0,
      pc: 0,
      error_handler: ErrorHandler::new(1, 0, String::from("Preprocesser"), con),
    };
  }
  
  pub fn process(&mut self) -> Vec<Expr> {
    let mut lb = -1;
    let mut lp = -1;
    for i in 0..self.tokens.len() {
      let (b, p) = self.count(i);
      if b >= 0 {
        lb = b;
      }
      if p >= 0 {
        lp = p;
      }
    
    }

    //Add trace back useing second list
    if self.pc > 0 {
      let fm = format!("too many parentheses at {:?}", self.pos[lp as usize]);
      self.error_handler.throw_error_untraced(String::from(fm));
    } else if self.pc < 0 {
      let fm = format!("missing parentheses at {:?}", self.pos[lp as usize]);
      self.error_handler.throw_error_untraced(String::from(fm));
    }
    if self.bc > 0 {
      let fm = format!("too many brackets at {:?}", self.pos[lb as usize]);
      self.error_handler.throw_error_untraced(String::from(fm));
    } else if self.bc < 0 {
      let fm = format!("missing brackets at {:?}", self.pos[lb as usize]);
      self.error_handler.throw_error_untraced(String::from(fm));
    }
    return self.tokens.clone();
  }

  fn count(&mut self, i: usize) -> (i32, i32) {
  let expr = self.tokens[i as usize].clone();
  let mut b: i32 = -1;
  let mut p: i32 = -1;
  if expr.get_value() == &Value::to_stringv("(") {
    self.pc += 1;
    p = i as i32;
  }
  else if expr.get_value() == &Value::to_stringv(")") {
    self.pc -= 1;
    p = i as i32;
  }
  else if expr.get_value() == &Value::to_stringv("{") {
    self.bc += 1;
    b = i as i32;
  }
  else if expr.get_value() == &Value::to_stringv("}") {
    self.bc -= 1;
    b = i as i32;
  }
    return (b, p);
  }
}