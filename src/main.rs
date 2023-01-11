#![allow(dead_code)]
#![allow(unused_mut)]
#[allow(unused_imports)]
use std::collections::HashMap;

use anyhow::{Context, Result};
use log::info;
use structopt::StructOpt;

use std::panic;

mod lang;
use expr::*;
use lang::*;

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
    let split: Vec<&str> = path.to_str().unwrap().split('.').collect();
    if !(split[split.len() - 1] == "lang") {
        println!(
            "File {:?} has a unexpected file extension of .{}",
            path,
            split[split.len() - 1]
        );
        panic!();
    }
    let content = std::fs::read_to_string(path)
        .with_context(|| format!("could not read file `{}`", "path"))?;

    let mut con: Vec<String> = content.clone().split('\n').map(|x| x.to_string()).collect();

    let mut tok = tokenizer::Tokenizer::new(content, con.clone());
    let (token_stream, pos) = tok.make_tokens(); // create preprocesser to check that all ( and { have matches and for () to make bool or int
    println!("stream: {:#?}", token_stream);
                                                 //println!("ts {:#?}", token_stream);
                                                 //output_pos(pos, content);
                                                 // return 2nd vec of positon in the input for each Expr in token_stream
    let mut prepro = preprocesser::Preprocesser::new(token_stream, &pos, con);
    let processed_stream = prepro.process();

    //error_handler.set(1, 0, String::from("Lexer"));

    let mut lex = lexer::Lexer::new(processed_stream);
    let tree = lex.tree();
    println!("tr {:?}", tree);
    let special_forms = specialforms::Specialforms::new();

    let mut eval = evaluate::Evaluate {
        special_forms: special_forms,
    };

    let mut scope: HashMap<String, Value> = HashMap::new();

    eval.evaluate(tree, &mut scope); // for each value check if needed +-/* or bool
    Ok(())
}

//x86_64-pc-windows-gnu
//x86_64-apple-darwin
//aarch64-unknown-linux-gnu

//cargo build --release --target {}
