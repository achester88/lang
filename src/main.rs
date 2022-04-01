//https://eloquentjavascript.net/12_language.html

use structopt::StructOpt;
use anyhow::{Context, Result};
use log::{info};//, warn};

#[path = "parser.rs"]
pub mod parser;

#[path = "evaluate.rs"]
pub mod evaluate;

#[path = "specialforms.rs"]
pub mod specialforms;

#[derive(StructOpt)]
struct Cli {
    /// The pattern to look for
    //pattern: String,
    /// The path to the file to read
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    env_logger::init();
    info!("starting up");
    let args = Cli::from_args();
    let path = args.path;//"test.txt";
    let content = std::fs::read_to_string(path)
        .with_context(|| format!("could not read file `{}`", "path"))?;
  /*
    for line in content.lines() {
    if line.contains(&args.pattern) {
        println!("{}", line);
    }
  }
*/
  //print_type_of(&content.lines().join(""));
  //let tokens = lexicalAnalysis::tokenization(&content);
  //let mut lines = content.lines();
  println!("_________Source_________\n{}\n________________________", content);
  //println!("{:#?}", parser::Parser::parse(&content));
  let special_forms = specialforms::Specialforms::new();
  let tree = parser::Parser::parse(&content);
  Ok(())
}

//args, scope