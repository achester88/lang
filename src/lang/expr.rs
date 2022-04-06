
//#[derive()]

  
#[derive(Debug, Clone)]
pub struct Expr {
  pub type_of: String,
  pub operator: Box<Option<Expr>>,
  pub args: Box<Option<Vec<Expr>>>,
  pub value: Box<Option<String>>
}

impl Expr  {
  pub fn to_string(&self) -> String {
    return format!("{{\ntype:{:?}\nop:{:?}\nargs:{:?}\nval:{:?}\n}}", self.type_of, self.operator, self.args, self.value);
  }
  
  pub fn addto(&mut self, elem: Expr) {
        self.args
            .get_or_insert_with(Vec::new)
            .push(elem);
    }
}
  
