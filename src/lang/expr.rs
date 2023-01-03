use std::fmt;
//#[derive()]

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum Value {
  Int(i32),
  //Float(f64),
  String(String),
  Bool(bool),
  Do,
  End,
  None
}

impl Value {
  pub fn to_string(self) -> String {
    match self {
        Value::Int(x) => x.to_string(),
        //Value::Float(x) => vec!("f:".to_string(), x.to_string()).join(" "),
        Value::String(x) => x.to_string(),
        Value::Bool(x) => x.to_string(),
        Value::Do => "do".to_string(),
        Value::End => "end".to_string(),
        Value::None => "none".to_string(),
    }
  }

  pub fn is_none(self) -> bool {
    self == Value::None
  }

  pub fn toString(s: &str) -> Value {
    Value::String(s.to_string())
  }
  pub fn toBool(s: &str) -> Value {
    match s {
      "true" => Value::Bool(true),
      "false" => Value::Bool(false),
      _ => Value::None,
    }
  }
  pub fn toInt(s: &str) -> Value {
    Value::Int(s.parse::<i32>().unwrap())
  }
}

impl fmt::Debug for Value {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Value::Int(x) => write!(f, "int: {}", x.to_string()),
      //Value::Float(x) => vec!("f:".to_string(), x.to_string()).join(" "),
      Value::String(x) =>write!(f, "string: \"{}\"", x.to_string()),
      Value::Bool(x) => write!(f, "bool: {}", x.to_string()),
      Value::Do => write!(f, "DO"),
      Value::End => write!(f, "END"),
      Value::None => write!(f, "NONE"),
  }
  }  
}

#[derive(Clone, PartialEq, Debug)]
pub enum Type {
  Do,
  Apply,
  Value,
  Word,
  None,
}

impl Type {
  pub fn to_string(self) -> String {
    match self {
        Type::Do => "do".to_string(),
        Type::Apply => "apply".to_string(),
        Type::Value => "value".to_string(),
        Type::Word => "word".to_string(),
        Type::None => "none".to_string(),
    }
  }
}

#[derive(Clone, PartialEq)]
pub struct Expr {
  pub type_of: Type,
  pub operator: Box<Option<Expr>>,
  pub args: Box<Option<Vec<Expr>>>,
  pub value: Value
}


/*
#[derive(Clone, PartialEq)]

pub struct Expr {
  pub type_of: String,
  pub operator: Box<Option<Expr>>,
  pub args: Box<Option<Vec<Expr>>>,
  pub value: Box<Option<String>>
}
*/

impl Expr  {

  pub fn get_value(&self) -> Value {
    return self.value.clone();
  }
  pub fn get_type(&self) -> Type {
    return self.type_of.clone();
  }
  pub fn get_args(&self) -> Vec<Expr> {
    return self.args.as_ref().as_ref().unwrap().clone();
  }
  pub fn get_operator(&self) -> Expr {
    return self.operator.as_ref().as_ref().unwrap().clone();
  }

  pub fn type_do(args: Vec<Expr>) -> Self {
    return Self {
      type_of: Type::Do,
      operator: Box::new(Some(Expr::word(Value::String("do".to_string())))),
      args: Box::new(Some(args)),
      value: Value::Do
    }; 
  }
  
  pub fn apply(op: Expr, args: Vec<Expr>) -> Self {
    return Self {
      type_of: Type::Apply,
      operator: Box::new(Some(op)),
      args: Box::new(Some(args)),
      value: Value::None
    }; 
  }
  
  pub fn value(val: Value) -> Self {
    return Self {
      type_of: Type::Value,
      operator: Box::new(None),
      args: Box::new(None),
      value: val
    };
  }

  pub fn sp_value(val: Value, kind: &str) -> Self {
    return Self {
      type_of: Type::Value,
      operator: Box::new(Some(Expr::value(Value::String(kind.to_string())))),
      args: Box::new(None),
      value: val
    };
  }
  
  pub fn word(val: Value) -> Self {
    return Self {
      type_of: Type::Word,
      operator: Box::new(None),
      args: Box::new(None),
      value: val
    };
  }

  pub fn sp_word(val: Value, kind: &str) -> Self {
    return Self {
      type_of: Type::Word,
      operator: Box::new(Some(Expr::value(Value::String(kind.to_string())))),
      args: Box::new(None),
      value: val
    };
  }
  
  pub fn empty() -> Self {
    return Self {
        type_of: Type::None, 
        value: Value::None,
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
    let mut op = Value::toString("");
    let mut t;
    if !self.operator.is_none() {
      t = self.get_operator();
       let v = t.get_value();
      op = v;
    }
    let mut args = vec!();
    if !self.args.is_none() {
      args = self.get_args();
    }
    let mut val = Value::toString("");
    if !self.value.clone().is_none() {
      let v = self.get_value();
      val = v;
    }
    write!(f, "{{ |{:?}|{:?}, {:?}agrs{:#?} }}", val, type_of, op, args)
  }  
}