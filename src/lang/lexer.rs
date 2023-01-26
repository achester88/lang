use crate::dict::Dict;
use crate::expr;
use expr::*;

pub struct Lexer {
    dict: Dict,
    source: Vec<Expr>,
    i: usize,
    key: String,
    match_c: char,
    is_control: bool,
    current: Vec<Expr>,
    list: Vec<Expr>,
    temp: Vec<Expr>,
    cond: Expr,
    ce: Expr,
    pc: i8,
    bc: i8,
}

impl Lexer {
    pub fn new(mut soruce: Vec<Expr>) -> Self {
        return Self {
            dict: Dict::new(),
            source: soruce,
            i: 0,
            key: String::from(""),
            match_c: '(',
            is_control: false,
            current: vec![],
            list: vec![],
            temp: vec![],
            cond: Expr::empty(),
            ce: Expr::empty(),
            pc: 0,
            bc: 0,
        };
    }

    pub fn tree(&mut self) -> Expr {
        //println!("=========tree===========s");
        return Expr::apply(Expr::word(Value::Do), self.parse());
    }

    pub fn parse(&mut self) -> Vec<Expr> {
        //println!("==========parse==========s");
        while self.i < self.source.len() {
            //if !self.ce.value.clone().is_none() {
            //println!("----------{:#?}----{}-----", self.ce, self.i);
            //}
            self.ce = self.source[self.i].clone();
            self.count();
            self.next();
            if !self.ce.value.clone().is_none() {
                //println!("----------{:#?}----{}-----", self.ce, self.i);
            }
            self.i += 1;
        }
        //println!("==========parse==========e");
        //println!("tree: {:#?}", self.list.clone());
        return self.list.clone();
    }

    pub fn count(&mut self) {
        //count ) and }
        if self.ce.get_value() == Value::toString("(") {
            self.pc += 1;
        }
        if self.ce.get_value() == Value::toString(")") {
            self.pc -= 1;
        }
        if self.ce.get_value() == Value::toString("{") {
            self.bc += 1;
        }
        if self.ce.get_value() == Value::toString("}") {
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
        if !self.ce.operator.is_none()
            && self.ce.get_operator().get_value() == Value::toString("key")
        {
            while self.ce.operator.is_none()
                || !(self.ce.get_operator().get_value() == Value::toString("end"))
            {
                //println!("e");
                self.ce = self.source[self.i].clone();
                self.temp.push(self.ce.clone());
                self.i += 1;
            }
            self.i -= 1;
            self.temp.pop();
            self.list
                .push(self.dict.getfns(self.temp[0].get_value().to_string())(
                    self.temp.clone(),
                ));
          self.temp = vec![];
        }
      
        if !self.ce.operator.is_none()
            && self.ce.get_operator().get_value() == Value::toString("ctrl")
        {
            self.key = self.ce.get_value().to_string();
            self.is_control = true;
        } else if self.value_match() {
            if self.is_control {
                if self.match_c == '{' {
                    self.match_c = '}';
                } else {
                    self.match_c = ')';
                }
            } else {
                self.key = self.source[self.i - 1].get_value().to_string(); //check if i > 0
                self.match_c = ')';
            }
        }
    }

    pub fn end(&mut self) {
        if self.value_match() {
            if self.is_control {
                if self.match_c == ')' {
                    self.match_c = '{';
                    self.cond = Expr::apply(Expr::word(Value::toString("eval_bool")), self.current.clone());
                    self.current = vec![];
                } else if self.match_c == '}' {
                    if self.bc == 0 {
                        //println!("{:#?}", self.current);
                        /////////////////
                        let mut lex = Lexer::new(self.current.clone());
                        self.temp.push(lex.tree());
                        self.temp.push(self.cond.clone());
                        //println!("TEMP: {:?}", self.temp.clone());                        self.temp.push(lex.tree());
                        /* for i in 0..self.current.len() {
                          temp.push(self.current[i].clone());
                        } */
                        if self.i + 1 < self.source.len()
                            && self.source[self.i + 1].clone().get_value() == Value::toString("else")
                        {
                            if self.i + 2 < self.source.len()
                            && self.source[self.i + 2].clone().get_value() == Value::toString("if") {
                            //println!("else in lex");
                            self.current = vec![];
                            self.is_control = true;
                            self.match_c = '(';
                            self.i += 2;
                            self.ce = self.source[self.i].clone();
                        } else {
                            self.current = vec![];
                            self.is_control = true;
                            self.match_c = '{';
                            self.cond = Expr::sp_value(Value::Bool(true), "bool");
                            self.i += 1;
                            self.ce = self.source[self.i].clone();
                        }
                        } else {
                          self.list.push(self.dict.getfns(self.key.clone())(self.temp.clone()));
                            self.current = vec![];
                            self.is_control = false;
                            self.match_c = '(';
                            self.temp = vec![];
                        }
                    } else {
                        //println!("#########{:#?}########", self.bc == 0);
                        self.current.push(self.ce.clone());
                    }
                }
            } else {
                //println!("-----####--{:#?}", self.current.clone());
              //println!("{:?}", self.key.clone());

              let mut expr: Expr;

              expr = match self.dict.getfn(self.key.clone()) {
                Ok(f) => f(self.current.clone()),
                Err(()) => { 
                println!("error");
                  Expr::apply(
                    Expr::word(Value::String(self.key.clone())),
                    self.current.clone(),
                  )}
              };
              
                self.list
                    .push(expr);
                self.current = vec![];
                self.match_c = '(';
              
            }
        } else {
            self.current.push(self.ce.clone());
        }
    }

    pub fn value_match(&mut self) -> bool {
        self.ce.get_value() == Value::String(self.match_c.to_string())
    }
}
