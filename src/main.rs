//https://eloquentjavascript.net/12_language.html
use std::collections::HashMap;

use structopt::StructOpt;
use anyhow::{Context, Result};
use log::{info};//, warn};

//#[path = "parser.rs"]
//mod parser;

//#[path = "evaluate.rs"]
//mod evaluate;

//#[path = "specialforms.rs"]
//mod specialforms;

mod lang;
use lang::*;

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

  println!("_________Source_________\n{}\n________________________", content);
  let special_forms = specialforms::Specialforms::new();
  let tree = parser::Parser::parse(content);
  let mut eval = evaluate::Evaluate { special_forms: special_forms};
  let mut scope: HashMap<String, String> = HashMap::new();
  eval.evaluate(tree, &mut scope);
  Ok(())
}

//args, scope