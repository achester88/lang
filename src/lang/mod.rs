pub mod evaluate;
pub mod expr;
pub mod specialforms;
pub mod lexer;
pub mod dict;
pub mod tokenizer;
pub mod errorhandler;
pub mod preprocesser;

pub fn output_pos(pos: Vec<(u32, u32)>, content: String) {
    //println!("{:?}", pos);
      let mut spaces: Vec<Vec<u32>> = vec![vec![]];
      let mut line: u32 = 0;
      for i in 0..pos.len() {
          let (cl, cc) = pos[i];
          if cl - 1 == line {
              spaces[line as usize].push(cc);
          } else {
              spaces.push(vec![cc]);
              line = cl - 1;
          }
      }
  
      line = 0;
      let mut c: u32 = 0;
      let ch: Vec<char> = content.chars().collect();
      for i in 0..ch.len() {
          print!("{}", ch[i]);
          if spaces[line as usize].contains(&(c+1)) {
              print!("|");
          }
          if ch[i] == '\n' {
              line += 1;
              c = 0;
          } else {
          c += 1;
          }
      }
      //print!("\n{:?}\n", spaces);
  }