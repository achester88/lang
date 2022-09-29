use std::fmt;
//#[derive()]

  
#[derive(Clone, PartialEq)]
pub struct Expr {
  pub type_of: String,
  pub operator: Box<Option<Expr>>,
  pub args: Box<Option<Vec<Expr>>>,
  pub value: Box<Option<String>>
}

impl Expr  {

  pub fn get_value(&self) -> &str {
    return &self.value.as_ref().as_ref().unwrap();
  }
  pub fn get_type(&self) -> &str {
    return &self.type_of;
  }
  pub fn get_args(&self) -> Vec<Expr> {
    return self.args.as_ref().as_ref().unwrap().clone();
  }
  pub fn get_operator(&self) -> Expr {
    return self.operator.as_ref().as_ref().unwrap().clone();
  }

  pub fn type_do(args: Vec<Expr>) -> Self {
    return Self {
      type_of: String::from("do"),
      operator: Box::new(Some(Expr::word("do"))),
      args: Box::new(Some(args)),
      value: Box::new(Some(String::from("do")))
    }; 
  }
  
  pub fn apply(op: Expr, args: Vec<Expr>) -> Self {
    return Self {
      type_of: String::from("apply"),
      operator: Box::new(Some(op)),
      args: Box::new(Some(args)),
      value: Box::new(None)
    }; 
  }
  
  pub fn value(val: &str) -> Self {
    return Self {
      type_of: String::from("value"),
      operator: Box::new(None),
      args: Box::new(None),
      value: Box::new(Some(String::from(val)))
    };
  }

  pub fn sp_value(val: &str, kind: &str) -> Self {
    return Self {
      type_of: String::from("value"),
      operator: Box::new(Some(Expr::value(kind))),
      args: Box::new(None),
      value: Box::new(Some(String::from(val)))
    };
  }
  
  pub fn word(val: &str) -> Self {
    return Self {
      type_of: String::from("word"),
      operator: Box::new(None),
      args: Box::new(None),
      value: Box::new(Some(String::from(val)))
    };
  }

  pub fn sp_word(val: &str, kind: &str) -> Self {
    return Self {
      type_of: String::from("word"),
      operator: Box::new(Some(Expr::value(kind))),
      args: Box::new(None),
      value: Box::new(Some(String::from(val)))
    };
  }
  
  pub fn empty() -> Self {
    return Self {
        type_of: String::from("empty"), 
        value: Box::new(None),
        operator: Box::new(None), 
        args: Box::new(None)
      };
  }
  
  pub fn to_string(&self) -> String {


      
    return format!("{{{:?} op:{:?} args {:#?} val:{:?}}}", self.type_of, self.operator, self.args, self.value);
  }
  
  pub fn addto(&mut self, elem: Expr) {
        self.args
            .get_or_insert_with(Vec::new)
            .push(elem);
    }
}
  
impl fmt::Debug for Expr {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let type_of = self.get_type();
    let mut op: String  = "".to_owned();
    let mut t;
    if !self.operator.is_none() {
      t = self.get_operator();
       let s: String = t.get_value().to_owned();
      op = s + ", ";
    }
    let mut args = vec!();
    if !self.args.is_none() {
      args = self.get_args();
    }
    let mut val: String = "".to_owned();
    if !self.value.is_none() {
      let s: String = self.get_value().to_owned();
      val = s + ", ";
    }
    write!(f, "{{ {}{}, {}agrs{:#?} }}", val, type_of, op, args)
  }  
}