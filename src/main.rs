#![allow(dead_code)]
#![allow(unused_mut)]
#[allow(unused_imports)]
use std::collections::HashMap;

use anyhow::{Context, Result};
use log::info;
use structopt::StructOpt;

use std::panic;

mod lang;
use lang::*;
use expr::*;

#[derive(StructOpt)]
struct Cli {
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    // /*
    panic::set_hook(Box::new(|_info| {
        // do nothing
    }));
    // */
    env_logger::init();
    info!("starting up");
    let args = Cli::from_args();
    let path = args.path;
    let content = std::fs::read_to_string(path)
        .with_context(|| format!("could not read file `{}`", "path"))?;
  
    let mut tok = tokenizer::Tokenizer::new(content);
    let token_stream = tok.make_tokens();// create preprocesser to check that all ( and { have matches and for () to make bool or int
    //println!("ts {:#?}", token_stream);

    println!("-----{}-----", 0);
    let mut c = 0;
    for i in 0..token_stream.clone().len() {
        let e = token_stream[i].clone();
        if(e.operator.is_none() || !(e.get_operator().get_value() ==  Value::toString("end"))) {
            println!("{}: {:?}", i, e);
            c += 1;
        }
    }
    println!("-----{}-----", c);
    let one = token_stream[37].clone();
    println!("none: {:?}", one.get_operator().get_value());
    println!("end: {}", !(one.get_operator().get_value() == Value::toString("end")));


    let mut lex = lexer::Lexer::new(token_stream);
    let tree = lex.tree();
    //println!("tr {:?}", tree);
    let special_forms = specialforms::Specialforms::new();

    let mut eval = evaluate::Evaluate {
        special_forms: special_forms,
    };

    let mut scope: HashMap<String, Value> = HashMap::new();

    eval.evaluate(tree, &mut scope); // for each value check if needed +-/* or bool
    Ok(())
}
