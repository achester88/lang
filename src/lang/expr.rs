
//#[derive()]

  
#[derive(Debug, Clone)]
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
  
