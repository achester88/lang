use crate::errorhandler;
use crate::expr;
use errorhandler::ErrorHandler;
use expr::*;
use regex::Regex;

#[derive(Debug, PartialEq, Clone)]
pub enum Type {
    String = 0,
    Number = 1,
    Punc = 2,
    Asgmt = 3,
    Par = 4,
    Sasgmt = 5,
    Comp = 6,
    Log = 7,
    Math = 8,
    Bool = 9,
    Word = 10,
    Ctrl = 11,
    Key = 12,
    NEWL = 13,
    Null = 14,
}

pub struct Tokenizer {
    pub i: usize,
    pub char: Vec<char>,
    pub lines_expr: Vec<Expr>,
    pub current: Vec<char>,
    pub current_type: Type,
    pub is_string: bool,
    pub error_handler: ErrorHandler,
}

impl Tokenizer {
    pub fn new(soruce: String) -> Self {
        let char: Vec<char> = soruce.chars().collect();
        let c = char[0];
        let ct = Type::Null; //self.get_type(&c.to_string());
        return Self {
            i: 1,
            char: char,
            lines_expr: vec![],
            current: vec![c],
            current_type: ct,
            is_string: false,
            error_handler: ErrorHandler::new(1,0, String::from("Tokenizer")),
        };
    }

    pub fn make_expr(&mut self, input: &str) -> Expr {
        let tp = self.get_type(input);
        return match tp {
            Type::String => Expr::value(Value::toString(input)),
            Type::Number => Expr::value(Value::toInt(input)),
            Type::Punc => Expr::sp_word(Value::toString(input), "punc"),
            Type::Asgmt => Expr::sp_word(Value::toString(input), "asgmt"),
            Type::Par => Expr::sp_value(Value::toString(input), "par"),
            Type::Sasgmt => Expr::sp_word(Value::toString(input), "sasgmt"),
            Type::Comp => Expr::sp_word(Value::toString(input), "comp"),
            Type::Log => Expr::sp_word(Value::toString(input), "log"),
            Type::Math => Expr::sp_word(Value::toString(input), "math"),
            Type::Bool => Expr::sp_value(Value::toBool(input), "bool"),
            Type::Ctrl => Expr::sp_word(Value::toString(input), "ctrl"),
            Type::Key => Expr::sp_value(Value::toString(input), "key"),
            _ => Expr::word(Value::toString(input)),
        };
    }

    pub fn get_type(&mut self, input: &str) -> Type {
        let mut mat: Vec<&str>;
        let is_number = Regex::new(r#"^\d+\b"#).unwrap();
        let is_word = Regex::new(r#"^[^\s,#"]+"#).unwrap();

        if input == "\n" {
            return Type::NEWL;
        }

        //Match Numbers ex: 10
        mat = is_number.find_iter(input).map(|x| x.as_str()).collect();
        if mat.len() > 0 {
            return Type::Number;
        }
        //Match words ex: print
        mat = is_word.find_iter(input).map(|x| x.as_str()).collect();
        if mat.len() > 0 {
            let punc = [",", ";", "."];
            let asgmt = ["="];
            let sasgmt = ["+=", "/=", "*=", "-="];
            let math = ["+", "-", "*", "/", "%"];
            let comp = ["==", "!=", "<", "<=", ">", ">=", "!"];
            let log = ["&&", "||"];
            let par = ["(", ")"];
            let bool = ["true", "false"];
            let control = ["if", "while"];
            let key = ["int", "bool"];

            if punc.contains(&mat[0]) {
                return Type::Punc;
            }
            if asgmt.contains(&mat[0]) {
                return Type::Asgmt;
            }
            if par.contains(&mat[0]) {
                return Type::Par;
            }
            if sasgmt.contains(&mat[0]) {
                return Type::Sasgmt;
            }
            if comp.contains(&mat[0]) {
                return Type::Comp;
            }
            if log.contains(&mat[0]) {
                return Type::Log;
            }
            if math.contains(&mat[0]) {
                return Type::Math;
            }
            if bool.contains(&mat[0]) {
                return Type::Bool;
            }
            if control.contains(&mat[0]) {
                return Type::Ctrl;
            }
            if key.contains(&mat[0]) {
                return Type::Key;
            }
            return Type::Word;
        } else {
            self.error_handler
                .throw_error(format!("Unexpected syntax: |{:#?}|", input));
            panic!();
        }
    }

    pub fn make_tokens(&mut self) -> Vec<Expr> {
        self.current_type = self.get_type(&self.char[0].to_string());
        self.error_handler.forwards(1);
        while self.i < self.char.len() {
            //println!("---------------");
            //println!("{}", self.i);
            self.advance();
            //println!("---------------");
        }
        if self.current.len() > 0 {
            self.push_current();
        }

        return self.lines_expr.clone();
    }

    fn push_current(&mut self) {
        //println!("{:#?}", self.current);
        let input: &str = &self.current.iter().collect::<String>();
        let expr = self.make_expr(input);
        self.lines_expr.push(expr);
        self.current = vec![];
        self.i_backward();
        self.current_type = Type::Null;
    }

    fn advance(&mut self) {
        if self.char[self.i] == '\r' {
        } else if self.char[self.i] == '\"' {
            if self.is_string {
                //println!("{:#?}", self.current);
                let input: &str = &self.current.iter().collect::<String>();
                let expr = Expr::value(Value::toString(input));
                self.lines_expr.push(expr);
                self.current = vec![];
                self.current_type = Type::Null;
            } else {
                if self.current.len() > 0 {
                    self.i_forward();
                    self.push_current();
                }
            }
            self.is_string = !self.is_string;
        } else if self.char[self.i] == ';' {
            if self.current.len() > 0 {
                self.i_forward();
                self.push_current();
            }
            self.lines_expr.push(expr::Expr::sp_value(Value::End, "end"));
        } else if self.char[self.i] == ',' {
            if self.current.len() > 0 {
                self.push_current();
            }
        } else if self.char[self.i] == ' ' {
            if self.current.len() > 0 {
                self.push_current();
            } else if self.char[self.i] == '/'
                && self.i + 1 < self.char.len()
                && self.char[self.i + 1] == '/'
            {
                if self.current.len() > 0 {
                    self.push_current();
                }
                while self.i + 1 < self.char.len() && self.char[self.i] != '\n' {
                    self.i_forward();
                }
            }
        } else {
            if self.is_string {
                self.current.push(self.char[self.i]);
            } else if self.current_type == Type::Null {
                self.current.push(self.char[self.i]);
                self.current_type = self.get_type(&self.char[self.i].to_string());
            } else {
                let t: Type = self.get_type(&self.char[self.i].to_string());
                if t == self.current_type {
                    self.current.push(self.char[self.i]);
                } else {
                    self.push_current();
                }
            }
        }
        self.i_forward();
    }

    fn i_forward(&mut self) {
        if self.char[self.i] == '\n' {
            self.error_handler.next_line();
        } else {
        self.error_handler.forwards(1);
        }
      self.i += 1;
    }
    fn i_backward(&mut self) {
        if self.char[self.i-1] == '\n' {
            self.error_handler.last_line();
        } else {
        self.error_handler.backwards(1);
        }
      self.i -= 1;
    }
}
